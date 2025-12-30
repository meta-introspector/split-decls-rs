// Generated macro for name_conflicts (function)
macro_rules! Depcrate_redundant_test_prefixname_conflicts {
() => {
// Module: crate::redundant_test_prefix
// Provides: {"name_conflicts"}
// Dependencies: {}
# [doc = " Checks whether removal of the `_test` prefix from the function name will cause a name conflict."] # [doc = ""] # [doc = " There should be no other function with the same name in the same module/scope. Also, there"] # [doc = " should not be any function call with the same name within the body of the function, to avoid"] # [doc = " recursion."] fn name_conflicts < 'tcx > (cx : & LateContext < 'tcx > , body : & 'tcx Body < '_ > , fn_name : Symbol) -> bool { let tcx = cx . tcx ; let id = body . id () . hir_id ; let (module , _module_span , _module_hir) = tcx . hir_get_module (tcx . parent_module (id)) ; if module . item_ids . iter () . any (| item | matches ! (tcx . hir_item (* item) . kind , hir :: ItemKind :: Fn { ident , .. } if ident . name == fn_name)) { return true ; } for_each_expr (cx , body , | expr | { if let ExprKind :: Path (qpath) = & expr . kind && let Some (def_id) = cx . qpath_res (qpath , expr . hir_id) . opt_def_id () && let Some (name) = tcx . opt_item_name (def_id) && name == fn_name { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
