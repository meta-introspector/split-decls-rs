// Generated macro for lit_to_expr (function)
macro_rules! Depcrate_attrlit_to_expr {
() => {
// Module: crate::attr
// Provides: {"lit_to_expr"}
// Dependencies: {}
# [doc = " Construct an expression from a literal."] fn lit_to_expr (lit : Lit) -> Expr { syn :: ExprLit { attrs : vec ! [] , lit } . into () }
};
}
