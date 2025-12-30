// Generated macro for invalid_escapes (function)
macro_rules! Depcrate_bytestr_testsinvalid_escapes {
() => {
// Module: crate::bytestr::tests
// Provides: {"invalid_escapes"}
// Dependencies: {}
# [test] fn invalid_escapes () { assert_err ! (ByteStringLit , r#"b"\a""# , UnknownEscape , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"foo\y""# , UnknownEscape , 5 .. 7) ; assert_err ! (ByteStringLit , r#"b"\"# , UnterminatedEscape , 2) ; assert_err ! (ByteStringLit , r#"b"\x""# , UnterminatedEscape , 2 .. 4) ; assert_err ! (ByteStringLit , r#"b"foo\x1""# , UnterminatedEscape , 5 .. 8) ; assert_err ! (ByteStringLit , r#"b" \xaj""# , InvalidXEscape , 3 .. 7) ; assert_err ! (ByteStringLit , r#"b"\xjbbaz""# , InvalidXEscape , 2 .. 6) ; }
};
}
