// Generated macro for invalid_escapes (function)
macro_rules! Depcrate_string_testsinvalid_escapes {
() => {
// Module: crate::string::tests
// Provides: {"invalid_escapes"}
// Dependencies: {}
# [test] fn invalid_escapes () { assert_err ! (StringLit , r#""\a""# , UnknownEscape , 1 .. 3) ; assert_err ! (StringLit , r#""foo\y""# , UnknownEscape , 4 .. 6) ; assert_err ! (StringLit , r#""\"# , UnterminatedEscape , 1) ; assert_err ! (StringLit , r#""\x""# , UnterminatedEscape , 1 .. 3) ; assert_err ! (StringLit , r#""🦊\x1""# , UnterminatedEscape , 5 .. 8) ; assert_err ! (StringLit , r#"" \xaj""# , InvalidXEscape , 2 .. 6) ; assert_err ! (StringLit , r#""నక్క\xjb""# , InvalidXEscape , 13 .. 17) ; }
};
}
