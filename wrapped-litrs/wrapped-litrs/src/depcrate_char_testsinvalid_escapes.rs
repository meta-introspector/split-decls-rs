// Generated macro for invalid_escapes (function)
macro_rules! Depcrate_char_testsinvalid_escapes {
() => {
// Module: crate::char::tests
// Provides: {"invalid_escapes"}
// Dependencies: {}
# [test] fn invalid_escapes () { assert_err ! (CharLit , r"'\a'" , UnknownEscape , 1 .. 3) ; assert_err ! (CharLit , r"'\y'" , UnknownEscape , 1 .. 3) ; assert_err ! (CharLit , r"'\" , UnterminatedEscape , 1) ; assert_err ! (CharLit , r"'\x'" , UnterminatedEscape , 1 .. 4) ; assert_err ! (CharLit , r"'\x1'" , InvalidXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xaj'" , InvalidXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xjb'" , InvalidXEscape , 1 .. 5) ; }
};
}
