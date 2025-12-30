// Generated macro for invalid_unicode_escapes (function)
macro_rules! Depcrate_char_testsinvalid_unicode_escapes {
() => {
// Module: crate::char::tests
// Provides: {"invalid_unicode_escapes"}
// Dependencies: {}
# [test] fn invalid_unicode_escapes () { assert_err ! (CharLit , r"'\u'" , UnicodeEscapeWithoutBrace , 1 .. 3) ; assert_err ! (CharLit , r"'\u '" , UnicodeEscapeWithoutBrace , 1 .. 3) ; assert_err ! (CharLit , r"'\u3'" , UnicodeEscapeWithoutBrace , 1 .. 3) ; assert_err ! (CharLit , r"'\u{'" , UnterminatedUnicodeEscape , 1 .. 5) ; assert_err ! (CharLit , r"'\u{12'" , UnterminatedUnicodeEscape , 1 .. 7) ; assert_err ! (CharLit , r"'\u{a0b'" , UnterminatedUnicodeEscape , 1 .. 8) ; assert_err ! (CharLit , r"'\u{a0_b  '" , UnterminatedUnicodeEscape , 1 .. 11) ; assert_err ! (CharLit , r"'\u{_}'" , InvalidStartOfUnicodeEscape , 4) ; assert_err ! (CharLit , r"'\u{_5f}'" , InvalidStartOfUnicodeEscape , 4) ; assert_err ! (CharLit , r"'\u{x}'" , NonHexDigitInUnicodeEscape , 4) ; assert_err ! (CharLit , r"'\u{0x}'" , NonHexDigitInUnicodeEscape , 5) ; assert_err ! (CharLit , r"'\u{3bx}'" , NonHexDigitInUnicodeEscape , 6) ; assert_err ! (CharLit , r"'\u{3b_x}'" , NonHexDigitInUnicodeEscape , 7) ; assert_err ! (CharLit , r"'\u{4x_}'" , NonHexDigitInUnicodeEscape , 5) ; assert_err ! (CharLit , r"'\u{1234567}'" , TooManyDigitInUnicodeEscape , 10) ; assert_err ! (CharLit , r"'\u{1234567}'" , TooManyDigitInUnicodeEscape , 10) ; assert_err ! (CharLit , r"'\u{1_23_4_56_7}'" , TooManyDigitInUnicodeEscape , 14) ; assert_err ! (CharLit , r"'\u{abcdef123}'" , TooManyDigitInUnicodeEscape , 10) ; assert_err ! (CharLit , r"'\u{110000}'" , InvalidUnicodeEscapeChar , 1 .. 11) ; }
};
}
