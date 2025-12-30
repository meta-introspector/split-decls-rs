// Generated macro for is_identity (function)
macro_rules! Depcrate_methods_filter_map_identityis_identity {
() => {
// Module: crate::methods::filter_map_identity
// Provides: {"is_identity"}
// Dependencies: {}
fn is_identity (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> Option < Applicability > { if is_expr_untyped_identity_function (cx , expr) { return Some (Applicability :: MachineApplicable) ; } if is_expr_identity_function (cx , expr) { return Some (Applicability :: Unspecified) ; } None }
};
}
