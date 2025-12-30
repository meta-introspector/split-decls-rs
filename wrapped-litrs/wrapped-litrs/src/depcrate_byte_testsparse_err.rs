// Generated macro for parse_err (function)
macro_rules! Depcrate_byte_testsparse_err {
() => {
// Module: crate::byte::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert_err ! (ByteLit , r"b''" , EmptyByteLiteral , None) ; assert_err ! (ByteLit , r"b' ''" , UnexpectedChar , 4 .. 5) ; assert_err ! (ByteLit , r"b'" , UnterminatedByteLiteral , None) ; assert_err ! (ByteLit , r"b'a" , UnterminatedByteLiteral , None) ; assert_err ! (ByteLit , r"b'\n" , UnterminatedByteLiteral , None) ; assert_err ! (ByteLit , r"b'\x35" , UnterminatedByteLiteral , None) ; assert_err ! (ByteLit , r"b'ab'" , OverlongByteLiteral , None) ; assert_err ! (ByteLit , r"b'a _'" , OverlongByteLiteral , None) ; assert_err ! (ByteLit , r"b'\n3'" , OverlongByteLiteral , None) ; assert_err ! (ByteLit , r"" , Empty , None) ; assert_err ! (ByteLit , r"b'''" , UnescapedSingleQuote , 2) ; assert_err ! (ByteLit , r"b''''" , UnescapedSingleQuote , 2) ; assert_err ! (ByteLit , "b'\n'" , UnescapedSpecialWhitespace , 2) ; assert_err ! (ByteLit , "b'\t'" , UnescapedSpecialWhitespace , 2) ; assert_err ! (ByteLit , "b'\r'" , UnescapedSpecialWhitespace , 2) ; assert_err ! (ByteLit , "b'న'" , NonAsciiInByteLiteral , 2) ; assert_err ! (ByteLit , "b'犬'" , NonAsciiInByteLiteral , 2) ; assert_err ! (ByteLit , "b'🦊'" , NonAsciiInByteLiteral , 2) ; }
};
}
