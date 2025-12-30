// Generated macro for is_in_test_function (function)
macro_rules! Depcrateis_in_test_function {
() => {
// Module: crate
// Provides: {"is_in_test_function"}
// Dependencies: {}
# [doc = " Checks if the function containing the given `HirId` is a `#[test]` function"] # [doc = ""] # [doc = " Note: Add `//@compile-flags: --test` to UI tests with a `#[test]` function"] pub fn is_in_test_function (tcx : TyCtxt < '_ > , id : HirId) -> bool { with_test_item_names (tcx , tcx . parent_module (id) , | names | { let node = tcx . hir_node (id) ; once ((id , node)) . chain (tcx . hir_parent_iter (id)) . any (| (_id , node) | { if let Node :: Item (item) = node && let ItemKind :: Fn { ident , .. } = item . kind { return names . binary_search (& ident . name) . is_ok () ; } false }) }) }
};
}
