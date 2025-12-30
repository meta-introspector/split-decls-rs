// Generated macro for is_relevant_trait (function)
macro_rules! Depcrate_attrs_utilsis_relevant_trait {
() => {
// Module: crate::attrs::utils
// Provides: {"is_relevant_trait"}
// Dependencies: {}
pub (super) fn is_relevant_trait (cx : & LateContext < '_ > , item : & TraitItem < '_ >) -> bool { match item . kind { TraitItemKind :: Fn (_ , TraitFn :: Required (_)) => true , TraitItemKind :: Fn (_ , TraitFn :: Provided (eid)) => { is_relevant_expr (cx , cx . tcx . typeck_body (eid) , cx . tcx . hir_body (eid) . value) } , _ => false , } }
};
}
