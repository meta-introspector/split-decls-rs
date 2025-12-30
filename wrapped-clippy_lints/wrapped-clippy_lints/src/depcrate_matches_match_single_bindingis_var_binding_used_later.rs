// Generated macro for is_var_binding_used_later (function)
macro_rules! Depcrate_matches_match_single_bindingis_var_binding_used_later {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"is_var_binding_used_later"}
// Dependencies: {}
fn is_var_binding_used_later (cx : & LateContext < '_ > , expr : & Expr < '_ > , arm : & Arm < '_ >) -> bool { let Node :: Stmt (stmt) = cx . tcx . parent_hir_node (expr . hir_id) else { return false ; } ; let Node :: Block (block) = cx . tcx . parent_hir_node (stmt . hir_id) else { return false ; } ; let mut identifiers = FxHashSet :: default () ; arm . pat . each_binding (| _ , _ , _ , ident | { identifiers . insert (ident . name) ; }) ; let mut visitor = VarBindingVisitor { cx , identifiers } ; block . stmts . iter () . skip_while (| s | s . hir_id != stmt . hir_id) . skip (1) . any (| stmt | matches ! (visitor . visit_stmt (stmt) , ControlFlow :: Break (()))) || block . expr . is_some_and (| expr | matches ! (visitor . visit_expr (expr) , ControlFlow :: Break (()))) }
};
}
