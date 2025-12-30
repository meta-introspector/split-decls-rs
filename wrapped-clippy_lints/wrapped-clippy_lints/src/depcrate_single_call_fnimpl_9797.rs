// Generated macro for impl_9797 (impl)
macro_rules! Depcrate_single_call_fnimpl_9797 {
() => {
// Module: crate::single_call_fn
// Provides: {"impl_9797"}
// Dependencies: {}
impl SingleCallFn { pub fn new (conf : & 'static Conf) -> Self { Self { avoid_breaking_exported_api : conf . avoid_breaking_exported_api , def_id_to_usage : FxIndexMap :: default () , } } fn is_function_allowed (& self , cx : & LateContext < '_ > , fn_def_id : LocalDefId , fn_hir_id : HirId , fn_span : Span ,) -> bool { (self . avoid_breaking_exported_api && cx . effective_visibilities . is_exported (fn_def_id)) || fn_span . in_external_macro (cx . sess () . source_map ()) || cx . tcx . hir_maybe_body_owned_by (fn_def_id) . is_none_or (| body | is_in_test_function (cx . tcx , body . value . hir_id)) || match cx . tcx . hir_node (fn_hir_id) { Node :: Item (item) => is_from_proc_macro (cx , item) , Node :: ImplItem (item) => is_from_proc_macro (cx , item) , Node :: TraitItem (item) => is_from_proc_macro (cx , item) , _ => true , } } }
};
}
