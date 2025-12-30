// Generated macro for impl_862 (impl)
macro_rules! Depcrate_usageimpl_862 {
() => {
// Module: crate::usage
// Provides: {"impl_862"}
// Dependencies: {}
impl < 'tcx > ParamBindingIdCollector { fn collect_binding_hir_ids (body : & 'tcx hir :: Body < 'tcx >) -> Vec < HirId > { let mut hir_ids : Vec < HirId > = Vec :: new () ; for param in body . params { let mut finder = ParamBindingIdCollector { binding_hir_ids : Vec :: new () , } ; finder . visit_param (param) ; for hir_id in & finder . binding_hir_ids { hir_ids . push (* hir_id) ; } } hir_ids } }
};
}
