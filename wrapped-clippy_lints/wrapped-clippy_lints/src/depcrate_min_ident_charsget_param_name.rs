// Generated macro for get_param_name (function)
macro_rules! Depcrate_min_ident_charsget_param_name {
() => {
// Module: crate::min_ident_chars
// Provides: {"get_param_name"}
// Dependencies: {}
fn get_param_name (impl_item : & ImplItem < '_ > , cx : & LateContext < '_ > , ident : Ident) -> Option < Symbol > { if let ImplItemImplKind :: Trait { trait_item_def_id : Ok (trait_item_def_id) , .. } = impl_item . impl_kind { let trait_param_names = cx . tcx . fn_arg_idents (trait_item_def_id) ; let ImplItemKind :: Fn (_ , body_id) = impl_item . kind else { return None ; } ; if let Some (param_index) = cx . tcx . hir_body_param_idents (body_id) . position (| param_ident | param_ident . is_some_and (| param_ident | param_ident . span == ident . span)) && let Some (trait_param_name) = trait_param_names . get (param_index) && let Some (trait_param_ident) = trait_param_name { return Some (trait_param_ident . name) ; } } None }
};
}
