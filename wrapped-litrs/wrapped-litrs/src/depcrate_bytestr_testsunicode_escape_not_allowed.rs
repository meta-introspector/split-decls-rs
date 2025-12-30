// Generated macro for unicode_escape_not_allowed (function)
macro_rules! Depcrate_bytestr_testsunicode_escape_not_allowed {
() => {
// Module: crate::bytestr::tests
// Provides: {"unicode_escape_not_allowed"}
// Dependencies: {}
# [test] fn unicode_escape_not_allowed () { assert_err ! (ByteStringLit , r#"b"\u{0}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{00}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{b}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{B}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{7e}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{E4}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{e4}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{fc}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{Fc}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{fC}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{FC}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{b10}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{B10}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{0b10}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{2764}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{1f602}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"\u{1F602}""# , UnicodeEscapeInByteLiteral , 2 .. 4) ; }
};
}
