macro_rules! deps {
    () => {
        StringLit!();
        InvalidToken!();
        FloatLit!();
        Span!();
        Literal!();
        TokenKind!();
    };
}

macro_rules! invalid_token_display {
    () => {
        deps!();
        # [cfg (feature = "proc-macro2")] # [test] fn invalid_token_display () { use crate :: { err :: TokenKind , InvalidToken } ; let span = crate :: err :: Span :: Two (proc_macro2 :: Span :: call_site ()) ; assert_eq ! (InvalidToken { actual : TokenKind :: StringLit , expected : TokenKind :: FloatLit , span , } . to_string () , r#"expected a float literal (e.g. `3.14`), but found a string literal (e.g. "Ferris")"# ,) ; assert_eq ! (InvalidToken { actual : TokenKind :: Punct , expected : TokenKind :: Literal , span , } . to_string () , r#"expected a literal, but found a punctuation character"# ,) ; }
    };
}

invalid_token_display!();