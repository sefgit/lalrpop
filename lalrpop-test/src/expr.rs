#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Expr<
    __TOKENS: IntoIterator<Item=Tok>,
>(
    scale: i32,
    __tokens: __TOKENS,
) -> Result<i32, Option<Tok>>
{
    let mut __tokens = __tokens.into_iter();
    let mut __tokens = __tokens.map(|t| ((), t, ()));
    let __lookahead = __tokens.next();
    match __parse__Expr::__state0(scale, None, __lookahead, &mut __tokens) {
        Err(None) => {
            Err(None)
        }
        Err(Some(__lookahead)) => {
            Err(Some(__lookahead.1))
        }
        Ok((_, Some(__lookahead), _)) => {
            Err(Some(__lookahead.1))
        }
        Ok((_, None, __parse__Expr::__Nonterminal::____Expr(__nt))) => {
            Ok(__nt)
        }
        Ok(_) => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;

    pub enum __Nonterminal<> {
        Factor(i32),
        Term(i32),
        Expr(i32),
        ____Expr(i32),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S2)
    //   "Num" -> Shift(S5)
    //
    //   Term -> S1
    //   Expr -> S4
    //   Factor -> S3
    pub fn __state0<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(scale, __lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state5(scale, __lookbehind, __lookahead, __tokens, __sym0));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(scale, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(scale, __lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 2
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S10)
    //   "Num" -> Shift(S9)
    //
    //   Factor -> S6
    //   Term -> S7
    //   Expr -> S8
    pub fn __state2<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(scale, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(scale, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S12)
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S11)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state12(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "+" -> Shift(S13)
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //   "-" -> Shift(S14)
    //
    pub fn __state4<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state13(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 5
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "+" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   EOF -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //
    pub fn __state5<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 6
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "*" -> Shift(S16)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S15)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //
    pub fn __state6<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state16(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state15(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state7<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 8
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "-" -> Shift(S18)
    //   "+" -> Shift(S19)
    //   ")" -> Shift(S17)
    //
    pub fn __state8<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state18(scale, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state19(scale, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state17(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   "/" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //   ")" -> Reduce(Term = "Num" => Call(ActionFn(7));)
    //
    pub fn __state9<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 10
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S10)
    //   "Num" -> Shift(S9)
    //
    //   Factor -> S6
    //   Term -> S7
    //   Expr -> S20
    pub fn __state10<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(scale, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(scale, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S5)
    //   "(" -> Shift(S2)
    //
    //   Term -> S21
    pub fn __state11<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state5(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S2)
    //   "Num" -> Shift(S5)
    //
    //   Term -> S22
    pub fn __state12<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state5(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S5)
    //   "(" -> Shift(S2)
    //
    //   Factor -> S23
    //   Term -> S1
    pub fn __state13<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state5(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S5)
    //   "(" -> Shift(S2)
    //
    //   Factor -> S24
    //   Term -> S1
    pub fn __state14<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state5(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S10)
    //
    //   Term -> S25
    pub fn __state15<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S10)
    //
    //   Term -> S26
    pub fn __state16<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state17<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 18
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S10)
    //   "Num" -> Shift(S9)
    //
    //   Term -> S7
    //   Factor -> S27
    pub fn __state18<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(scale, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 19
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S10)
    //   "Num" -> Shift(S9)
    //
    //   Factor -> S28
    //   Term -> S7
    pub fn __state19<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state10(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(scale, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(scale, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S29)
    //   "+" -> Shift(S19)
    //   "-" -> Shift(S18)
    //
    pub fn __state20<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state29(scale, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state19(scale, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state18(scale, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state21<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 22
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state22<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 23
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S11)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S12)
    //
    pub fn __state23<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state12(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S11)
    //   "*" -> Shift(S12)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //
    pub fn __state24<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state12(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state25<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 26
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state26<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 27
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S16)
    //   "/" -> Shift(S15)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //
    pub fn __state27<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state16(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state15(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 28
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "*" -> Shift(S16)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S15)
    //
    pub fn __state28<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state16(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state15(scale, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state29<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

pub fn __action0<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l - r
}

pub fn __action2<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l + r
}

pub fn __action3<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action4<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l * r
}

pub fn __action5<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l / r
}

pub fn __action6<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action7<
>(
    scale: i32,
    n: i32,
) -> i32
{
    n * scale
}

pub fn __action8<
>(
    scale: i32,
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32
{
    (__0)
}
