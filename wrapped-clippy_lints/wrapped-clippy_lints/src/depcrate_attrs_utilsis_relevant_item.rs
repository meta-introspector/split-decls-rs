// Generated macro for is_relevant_item (function)
macro_rules! Depcrate_attrs_utilsis_relevant_item {
() => {
// Module: crate::attrs::utils
// Provides: {"is_relevant_item"}
// Dependencies: {}
pub (super) fn is_relevant_item (cx : & LateContext < '_ > , item : & Item < '_ >) -> bool { if let ItemKind :: Fn { body : eid , .. } = item . kind { is_relevant_expr (cx , cx . tcx . typeck_body (eid) , cx . tcx . hir_body (eid) . value) } else { false } }
};
}
