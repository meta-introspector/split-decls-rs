// Generated macro for impl_2137 (impl)
macro_rules! Depcrate_extra_unused_type_parametersimpl_2137 {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"impl_2137"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ExtraUnusedTypeParameters { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if let ItemKind :: Fn { generics , body : body_id , .. } = item . kind && ! generics . params . is_empty () && ! is_empty_body (cx , body_id) && (! self . avoid_breaking_exported_api || ! cx . effective_visibilities . is_exported (item . owner_id . def_id)) && ! item . span . in_external_macro (cx . sess () . source_map ()) && ! is_from_proc_macro (cx , item) { let mut walker = TypeWalker :: new (cx , generics) ; walk_item (& mut walker , item) ; walker . emit_lint () ; } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx ImplItem < 'tcx >) { if let ImplItemKind :: Fn (.. , body_id) = item . kind && ! item . generics . params . is_empty () && trait_ref_of_method (cx , item . owner_id) . is_none () && ! is_empty_body (cx , body_id) && (! self . avoid_breaking_exported_api || ! cx . effective_visibilities . is_exported (item . owner_id . def_id)) && ! item . span . in_external_macro (cx . sess () . source_map ()) && ! is_from_proc_macro (cx , item) { let mut walker = TypeWalker :: new (cx , item . generics) ; walk_impl_item (& mut walker , item) ; walker . emit_lint () ; } } }
};
}
