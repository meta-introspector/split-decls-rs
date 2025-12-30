// Generated macro for parse_err (function)
macro_rules! Depcrate_char_testsparse_err {
() => {
// Module: crate::char::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert_err ! (CharLit , r"''" , EmptyCharLiteral , None) ; assert_err ! (CharLit , r"' ''" , UnexpectedChar , 3) ; assert_err ! (CharLit , r"'" , UnterminatedCharLiteral , None) ; assert_err ! (CharLit , r"'a" , UnterminatedCharLiteral , None) ; assert_err ! (CharLit , r"'\n" , UnterminatedCharLiteral , None) ; assert_err ! (CharLit , r"'\x35" , UnterminatedCharLiteral , None) ; assert_err ! (CharLit , r"'ab'" , OverlongCharLiteral , None) ; assert_err ! (CharLit , r"'a _'" , OverlongCharLiteral , None) ; assert_err ! (CharLit , r"'\n3'" , OverlongCharLiteral , None) ; assert_err ! (CharLit , r"" , Empty , None) ; assert_err ! (CharLit , r"'''" , UnescapedSingleQuote , 1) ; assert_err ! (CharLit , r"''''" , UnescapedSingleQuote , 1) ; assert_err ! (CharLit , "'\n'" , UnescapedSpecialWhitespace , 1) ; assert_err ! (CharLit , "'\t'" , UnescapedSpecialWhitespace , 1) ; assert_err ! (CharLit , "'\r'" , UnescapedSpecialWhitespace , 1) ; }
};
}
