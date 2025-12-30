// Generated macro for is_test_function (function)
macro_rules! Depcrateis_test_function {
() => {
// Module: crate
// Provides: {"is_test_function"}
// Dependencies: {}
# [doc = " Checks if `fn_def_id` has a `#[test]` attribute applied"] # [doc = ""] # [doc = " This only checks directly applied attributes. To see if a node has a parent function marked with"] # [doc = " `#[test]` use [`is_in_test_function`]."] # [doc = ""] # [doc = " Note: Add `//@compile-flags: --test` to UI tests with a `#[test]` function"] pub fn is_test_function (tcx : TyCtxt < '_ > , fn_def_id : LocalDefId) -> bool { let id = tcx . local_def_id_to_hir_id (fn_def_id) ; if let Node :: Item (item) = tcx . hir_node (id) && let ItemKind :: Fn { ident , .. } = item . kind { with_test_item_names (tcx , tcx . parent_module (id) , | names | { names . binary_search (& ident . name) . is_ok () }) } else { false } }
};
}
