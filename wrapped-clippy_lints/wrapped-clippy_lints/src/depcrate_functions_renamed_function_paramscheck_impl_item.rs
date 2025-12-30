// Generated macro for check_impl_item (function)
macro_rules! Depcrate_functions_renamed_function_paramscheck_impl_item {
() => {
// Module: crate::functions::renamed_function_params
// Provides: {"check_impl_item"}
// Dependencies: {}
pub (super) fn check_impl_item (cx : & LateContext < '_ > , item : & ImplItem < '_ > , ignored_traits : & DefIdSet) { if ! item . span . from_expansion () && let ImplItemKind :: Fn (_ , body_id) = item . kind && let parent_node = cx . tcx . parent_hir_node (item . hir_id ()) && let Node :: Item (parent_item) = parent_node && let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = & parent_item . kind && let Some (did) = cx . tcx . trait_item_of (item . owner_id) && ! is_from_ignored_trait (& of_trait . trait_ref , ignored_traits) { let mut param_idents_iter = cx . tcx . hir_body_param_idents (body_id) ; let mut default_param_idents_iter = cx . tcx . fn_arg_idents (did) . iter () . copied () ; let renames = RenamedFnArgs :: new (& mut default_param_idents_iter , & mut param_idents_iter) ; if ! renames . 0 . is_empty () { let multi_span = renames . multi_span () ; let plural = if renames . 0 . len () == 1 { "" } else { "s" } ; span_lint_and_then (cx , RENAMED_FUNCTION_PARAMS , multi_span , format ! ("renamed function parameter{plural} of trait impl") , | diag | { diag . multipart_suggestion (format ! ("consider using the default name{plural}") , renames . 0 , Applicability :: Unspecified ,) ; } ,) ; } } }
};
}
