// Generated macro for make_unop (function)
macro_rules! Depcrate_suggmake_unop {
() => {
// Module: crate::sugg
// Provides: {"make_unop"}
// Dependencies: {}
# [doc = " Builds the string for `<op><expr>` adding parenthesis when necessary."] # [doc = ""] # [doc = " For convenience, the operator is taken as a string because all unary"] # [doc = " operators have the same precedence."] pub fn make_unop (op : & str , expr : Sugg < '_ >) -> Sugg < 'static > { Sugg :: MaybeParen (format ! ("{op}{}" , expr . maybe_inner_paren ()) . into ()) }
};
}
