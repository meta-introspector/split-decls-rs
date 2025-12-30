// Generated macro for parse_err (function)
macro_rules! Depcrate_bytestr_testsparse_err {
() => {
// Module: crate::bytestr::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert_err ! (ByteStringLit , r#"b""# , UnterminatedString , None) ; assert_err ! (ByteStringLit , r#"b"cat"# , UnterminatedString , None) ; assert_err ! (ByteStringLit , r#"b"Jurgen"# , UnterminatedString , None) ; assert_err ! (ByteStringLit , r#"b"foo bar baz"# , UnterminatedString , None) ; assert_err ! (ByteStringLit , r#"b"fox"peter""# , InvalidSuffix , 6) ; assert_err ! (ByteStringLit , r###"br#"foo "# bar"#"### , UnexpectedChar , 10) ; assert_err ! (ByteStringLit , "b\"\r\"" , CarriageReturn , 2) ; assert_err ! (ByteStringLit , "b\"fo\rx\"" , CarriageReturn , 4) ; assert_err ! (ByteStringLit , "br\"\r\"" , CarriageReturn , 3) ; assert_err ! (ByteStringLit , "br\"fo\rx\"" , CarriageReturn , 5) ; assert_err ! (ByteStringLit , "b\"a\\\r\"" , UnknownEscape , 3 .. 5) ; assert_err ! (ByteStringLit , "br\"a\\\r\"" , CarriageReturn , 5) ; assert_err ! (ByteStringLit , r##"br####""## , UnterminatedRawString , None) ; assert_err ! (ByteStringLit , r#####"br##"foo"#bar"##### , UnterminatedRawString , None) ; assert_err ! (ByteStringLit , r##"br####"## , InvalidLiteral , None) ; assert_err ! (ByteStringLit , r##"br####x"## , InvalidLiteral , None) ; }
};
}
