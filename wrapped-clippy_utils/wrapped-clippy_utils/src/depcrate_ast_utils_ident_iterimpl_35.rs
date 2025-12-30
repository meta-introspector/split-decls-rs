// Generated macro for impl_35 (impl)
macro_rules! Depcrate_ast_utils_ident_iterimpl_35 {
() => {
// Module: crate::ast_utils::ident_iter
// Provides: {"impl_35"}
// Dependencies: {}
impl From < & Expr > for IdentIter { fn from (expr : & Expr) -> Self { let mut visitor = IdentCollector :: default () ; walk_expr (& mut visitor , expr) ; IdentIter (visitor . 0 . into_iter ()) } }
};
}
