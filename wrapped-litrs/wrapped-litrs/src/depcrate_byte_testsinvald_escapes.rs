// Generated macro for invald_escapes (function)
macro_rules! Depcrate_byte_testsinvald_escapes {
() => {
// Module: crate::byte::tests
// Provides: {"invald_escapes"}
// Dependencies: {}
# [test] fn invald_escapes () { assert_err ! (ByteLit , r"b'\a'" , UnknownEscape , 2 .. 4) ; assert_err ! (ByteLit , r"b'\y'" , UnknownEscape , 2 .. 4) ; assert_err ! (ByteLit , r"b'\" , UnterminatedEscape , 2 .. 3) ; assert_err ! (ByteLit , r"b'\x'" , UnterminatedEscape , 2 .. 5) ; assert_err ! (ByteLit , r"b'\x1'" , InvalidXEscape , 2 .. 6) ; assert_err ! (ByteLit , r"b'\xaj'" , InvalidXEscape , 2 .. 6) ; assert_err ! (ByteLit , r"b'\xjb'" , InvalidXEscape , 2 .. 6) ; }
};
}
