// Generated macro for check_trait_item (function)
macro_rules! Depcrate_functions_ref_optioncheck_trait_item {
() => {
// Module: crate::functions::ref_option
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item < 'a > (cx : & LateContext < 'a > , trait_item : & hir :: TraitItem < 'a > , avoid_breaking_exported_api : bool ,) { if ! trait_item . span . in_external_macro (cx . sess () . source_map ()) && let hir :: TraitItemKind :: Fn (ref sig , _) = trait_item . kind && ! (avoid_breaking_exported_api && cx . effective_visibilities . is_exported (trait_item . owner_id . def_id)) && ! is_from_proc_macro (cx , trait_item) { let def_id = trait_item . owner_id . def_id ; let ty_sig = cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () ; check_fn_sig (cx , sig . decl , sig . span , ty_sig) ; } }
};
}
