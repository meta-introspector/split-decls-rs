// Generated macro for get_ident (function)
macro_rules! Depcrate_suspicious_operation_groupingsget_ident {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"get_ident"}
// Dependencies: {}
fn get_ident (expr : & Expr , location : IdentLocation) -> Option < Ident > { IdentIter :: from (expr) . nth (location . index) }
};
}
