// Generated macro for tokenize_with_text (function)
macro_rules! Depcratetokenize_with_text {
() => {
// Module: crate
// Provides: {"tokenize_with_text"}
// Dependencies: {}
# [doc = " Tokenizes the input while keeping the text associated with each token."] pub fn tokenize_with_text (s : & str) -> impl Iterator < Item = (TokenKind , & str , InnerSpan) > { let mut pos = 0 ; tokenize (s , FrontmatterAllowed :: No) . map (move | t | { let end = pos + t . len ; let range = pos as usize .. end as usize ; let inner = InnerSpan :: new (range . start , range . end) ; pos = end ; (t . kind , s . get (range) . unwrap_or_default () , inner) }) }
};
}
