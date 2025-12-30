// Generated macro for invalid_escapes (function)
macro_rules! Depcrate_cstr_testsinvalid_escapes {
() => {
// Module: crate::cstr::tests
// Provides: {"invalid_escapes"}
// Dependencies: {}
# [test] fn invalid_escapes () { assert_err ! (CStringLit , r#"c"\a""# , UnknownEscape , 2 .. 4) ; assert_err ! (CStringLit , r#"c"foo\y""# , UnknownEscape , 5 .. 7) ; assert_err ! (CStringLit , r#"c"\"# , UnterminatedEscape , 2) ; assert_err ! (CStringLit , r#"c"\x""# , UnterminatedEscape , 2 .. 4) ; assert_err ! (CStringLit , r#"c"foo\x1""# , UnterminatedEscape , 5 .. 8) ; assert_err ! (CStringLit , r#"c" \xaj""# , InvalidXEscape , 3 .. 7) ; assert_err ! (CStringLit , r#"c"\xjbbaz""# , InvalidXEscape , 2 .. 6) ; }
};
}
