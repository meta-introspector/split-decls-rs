// Generated macro for SpanlessEqCallback (type)
macro_rules! Depcrate_hir_utilsSpanlessEqCallback {
() => {
// Module: crate::hir_utils
// Provides: {"SpanlessEqCallback"}
// Dependencies: {}
# [doc = " Callback that is called when two expressions are not equal in the sense of `SpanlessEq`, but"] # [doc = " other conditions would make them equal."] type SpanlessEqCallback < 'a > = dyn FnMut (& Expr < '_ > , & Expr < '_ >) -> bool + 'a ;
};
}
