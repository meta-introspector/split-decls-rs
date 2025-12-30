// Generated macro for impl_865 (impl)
macro_rules! Depcrate_usageimpl_865 {
() => {
// Module: crate::usage
// Provides: {"impl_865"}
// Dependencies: {}
impl < 'a , 'tcx > BindingUsageFinder < 'a , 'tcx > { pub fn are_params_used (cx : & 'a LateContext < 'tcx > , body : & 'tcx hir :: Body < 'tcx >) -> bool { let mut finder = BindingUsageFinder { cx , binding_ids : ParamBindingIdCollector :: collect_binding_hir_ids (body) , } ; finder . visit_body (body) . is_break () } }
};
}
