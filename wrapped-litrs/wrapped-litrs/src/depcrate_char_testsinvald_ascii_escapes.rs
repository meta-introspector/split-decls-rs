// Generated macro for invald_ascii_escapes (function)
macro_rules! Depcrate_char_testsinvald_ascii_escapes {
() => {
// Module: crate::char::tests
// Provides: {"invald_ascii_escapes"}
// Dependencies: {}
# [test] fn invald_ascii_escapes () { assert_err ! (CharLit , r"'\x80'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\x81'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\x8a'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\x8F'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xa0'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xB0'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xc3'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xDf'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xff'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xfF'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xFf'" , NonAsciiXEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\xFF'" , NonAsciiXEscape , 1 .. 5) ; }
};
}
