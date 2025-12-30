// Generated macro for parse_err (function)
macro_rules! Depcrate_cstr_testsparse_err {
() => {
// Module: crate::cstr::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert_err ! (CStringLit , r#"c""# , UnterminatedString , None) ; assert_err ! (CStringLit , r#"c"cat"# , UnterminatedString , None) ; assert_err ! (CStringLit , r#"c"Jurgen"# , UnterminatedString , None) ; assert_err ! (CStringLit , r#"c"foo bar baz"# , UnterminatedString , None) ; assert_err ! (CStringLit , r#"c"fox"peter""# , InvalidSuffix , 6) ; assert_err ! (CStringLit , r###"cr#"foo "# bar"#"### , UnexpectedChar , 10) ; assert_err ! (CStringLit , "c\"\r\"" , CarriageReturn , 2) ; assert_err ! (CStringLit , "c\"fo\rx\"" , CarriageReturn , 4) ; assert_err ! (CStringLit , "cr\"\r\"" , CarriageReturn , 3) ; assert_err ! (CStringLit , "cr\"fo\rx\"" , CarriageReturn , 5) ; assert_err ! (CStringLit , "c\"a\\\r\"" , UnknownEscape , 3 .. 5) ; assert_err ! (CStringLit , "cr\"a\\\r\"" , CarriageReturn , 5) ; assert_err ! (CStringLit , r##"cr####""## , UnterminatedRawString , None) ; assert_err ! (CStringLit , r#####"cr##"foo"#bar"##### , UnterminatedRawString , None) ; assert_err ! (CStringLit , r##"cr####"## , InvalidLiteral , None) ; assert_err ! (CStringLit , r##"cr####x"## , InvalidLiteral , None) ; }
};
}
