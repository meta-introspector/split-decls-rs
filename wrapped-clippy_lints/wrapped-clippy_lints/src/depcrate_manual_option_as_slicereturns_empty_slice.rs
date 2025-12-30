// Generated macro for returns_empty_slice (function)
macro_rules! Depcrate_manual_option_as_slicereturns_empty_slice {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"returns_empty_slice"}
// Dependencies: {}
fn returns_empty_slice (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Path (_) => expr . res (cx) . is_diag_item (cx , sym :: default_fn) , ExprKind :: Closure (cl) => is_empty_slice (cx , cx . tcx . hir_body (cl . body) . value) , _ => false , } }
};
}
