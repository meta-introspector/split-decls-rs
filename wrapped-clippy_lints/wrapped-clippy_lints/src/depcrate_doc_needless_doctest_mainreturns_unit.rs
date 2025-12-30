// Generated macro for returns_unit (function)
macro_rules! Depcrate_doc_needless_doctest_mainreturns_unit {
() => {
// Module: crate::doc::needless_doctest_main
// Provides: {"returns_unit"}
// Dependencies: {}
fn returns_unit < 'a > (mut tokens : impl Iterator < Item = (TokenKind , & 'a str , InnerSpan) >) -> bool { let mut next = | | tokens . next () . map_or (TokenKind :: Whitespace , | (kind , ..) | kind) ; match next () { TokenKind :: OpenBrace => true , TokenKind :: Minus => { next () == TokenKind :: Gt && next () == TokenKind :: OpenParen && next () == TokenKind :: CloseParen && next () == TokenKind :: OpenBrace } , _ => false , } }
};
}
