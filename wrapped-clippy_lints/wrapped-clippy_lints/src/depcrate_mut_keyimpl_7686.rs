// Generated macro for impl_7686 (impl)
macro_rules! Depcrate_mut_keyimpl_7686 {
() => {
// Module: crate::mut_key
// Provides: {"impl_7686"}
// Dependencies: {}
impl < 'tcx > MutableKeyType < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , conf : & 'static Conf) -> Self { Self { interior_mut : InteriorMut :: without_pointers (tcx , & conf . ignore_interior_mutability) , } } fn check_sig (& mut self , cx : & LateContext < 'tcx > , fn_def_id : LocalDefId , decl : & hir :: FnDecl < 'tcx >) { let fn_sig = cx . tcx . fn_sig (fn_def_id) . instantiate_identity () ; for (hir_ty , ty) in iter :: zip (decl . inputs , fn_sig . inputs () . skip_binder ()) { self . check_ty_ (cx , hir_ty . span , * ty) ; } self . check_ty_ (cx , decl . output . span () , cx . tcx . instantiate_bound_regions_with_erased (fn_sig . output ()) ,) ; } fn check_ty_ (& mut self , cx : & LateContext < 'tcx > , span : Span , ty : Ty < 'tcx >) { let ty = ty . peel_refs () ; if let ty :: Adt (def , args) = ty . kind () && matches ! (cx . tcx . get_diagnostic_name (def . did ()) , Some (sym :: HashMap | sym :: BTreeMap | sym :: HashSet | sym :: BTreeSet)) { let subst_ty = args . type_at (0) ; if let Some (chain) = self . interior_mut . interior_mut_ty_chain (cx , subst_ty) { span_lint_and_then (cx , MUTABLE_KEY_TYPE , span , "mutable key type" , | diag | { for ty in chain . iter () . rev () { diag . note (with_forced_trimmed_paths ! (format ! ("... because it contains `{ty}`, which has interior mutability"))) ; } }) ; } } } }
};
}
