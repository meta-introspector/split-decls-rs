// Generated macro for variable (function)
macro_rules! Depcrate_constructvariable {
() => {
// Module: crate::construct
// Provides: {"variable"}
// Dependencies: {}
pub (crate) fn variable (name : & str) -> Expr { let ident = Ident :: new (name , Span2 :: call_site ()) ; parse_quote ! (# ident) }
};
}
