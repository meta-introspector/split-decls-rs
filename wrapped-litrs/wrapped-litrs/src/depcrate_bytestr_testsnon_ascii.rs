// Generated macro for non_ascii (function)
macro_rules! Depcrate_bytestr_testsnon_ascii {
() => {
// Module: crate::bytestr::tests
// Provides: {"non_ascii"}
// Dependencies: {}
# [test] fn non_ascii () { assert_err ! (ByteStringLit , r#"b"న""# , NonAsciiInByteLiteral , 2) ; assert_err ! (ByteStringLit , r#"b"foo犬""# , NonAsciiInByteLiteral , 5) ; assert_err ! (ByteStringLit , r#"b"x🦊baz""# , NonAsciiInByteLiteral , 3) ; assert_err ! (ByteStringLit , r#"br"న""# , NonAsciiInByteLiteral , 3) ; assert_err ! (ByteStringLit , r#"br"foo犬""# , NonAsciiInByteLiteral , 6) ; assert_err ! (ByteStringLit , r#"br"x🦊baz""# , NonAsciiInByteLiteral , 4) ; }
};
}
