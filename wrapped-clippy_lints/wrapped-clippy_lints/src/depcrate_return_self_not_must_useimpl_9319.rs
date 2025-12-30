// Generated macro for impl_9319 (impl)
macro_rules! Depcrate_return_self_not_must_useimpl_9319 {
() => {
// Module: crate::return_self_not_must_use
// Provides: {"impl_9319"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ReturnSelfNotMustUse { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < 'tcx > , _ : & 'tcx Body < 'tcx > , span : Span , fn_def : LocalDefId ,) { if matches ! (kind , FnKind :: Method (_ , _)) && let Some (impl_def) = cx . tcx . impl_of_assoc (fn_def . to_def_id ()) && cx . tcx . trait_id_of_impl (impl_def) . is_none () { let hir_id = cx . tcx . local_def_id_to_hir_id (fn_def) ; check_method (cx , decl , fn_def , span , hir_id . expect_owner ()) ; } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx TraitItem < 'tcx >) { if let TraitItemKind :: Fn (ref sig , _) = item . kind { check_method (cx , sig . decl , item . owner_id . def_id , item . span , item . owner_id) ; } } }
};
}
