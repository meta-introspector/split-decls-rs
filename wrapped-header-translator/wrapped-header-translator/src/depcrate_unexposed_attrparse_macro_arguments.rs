// Generated macro for parse_macro_arguments (function)
macro_rules! Depcrate_unexposed_attrparse_macro_arguments {
() => {
// Module: crate::unexposed_attr
// Provides: {"parse_macro_arguments"}
// Dependencies: {}
pub (crate) fn parse_macro_arguments (tokens : & [Token < '_ >]) -> TokenStream { let Some ((start , tokens)) = tokens . split_first () else { return TokenStream :: new () ; } ; assert_eq ! (start . get_kind () , TokenKind :: Punctuation) ; assert_eq ! (start . get_spelling () , "(") ; let (end , tokens) = tokens . split_last () . expect ("tokens to have parentheses") ; assert_eq ! (end . get_kind () , TokenKind :: Punctuation) ; assert_eq ! (end . get_spelling () , ")") ; tokens . iter () . map (| token | token . get_spelling ()) . collect :: < Vec < _ > > () . join ("") . parse () . expect ("invalid tokenstream") }
};
}
