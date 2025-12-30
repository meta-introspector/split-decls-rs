// Generated macro for has_ref_mut_self_method (function)
macro_rules! Depcrate_needless_borrows_for_generic_argshas_ref_mut_self_method {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"has_ref_mut_self_method"}
// Dependencies: {}
fn has_ref_mut_self_method (cx : & LateContext < '_ > , trait_def_id : DefId) -> bool { cx . tcx . associated_items (trait_def_id) . in_definition_order () . any (| assoc_item | { if assoc_item . is_method () { let self_ty = cx . tcx . fn_sig (assoc_item . def_id) . instantiate_identity () . skip_binder () . inputs () [0] ; matches ! (self_ty . kind () , ty :: Ref (_ , _ , Mutability :: Mut)) } else { false } }) }
};
}
