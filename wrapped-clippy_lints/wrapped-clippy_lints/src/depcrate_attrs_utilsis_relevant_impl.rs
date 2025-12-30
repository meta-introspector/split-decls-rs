// Generated macro for is_relevant_impl (function)
macro_rules! Depcrate_attrs_utilsis_relevant_impl {
() => {
// Module: crate::attrs::utils
// Provides: {"is_relevant_impl"}
// Dependencies: {}
pub (super) fn is_relevant_impl (cx : & LateContext < '_ > , item : & ImplItem < '_ >) -> bool { match item . kind { ImplItemKind :: Fn (_ , eid) => is_relevant_expr (cx , cx . tcx . typeck_body (eid) , cx . tcx . hir_body (eid) . value) , _ => false , } }
};
}
