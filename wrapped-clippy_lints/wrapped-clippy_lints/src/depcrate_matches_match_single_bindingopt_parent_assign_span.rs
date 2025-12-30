// Generated macro for opt_parent_assign_span (function)
macro_rules! Depcrate_matches_match_single_bindingopt_parent_assign_span {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"opt_parent_assign_span"}
// Dependencies: {}
# [doc = " Returns true if the `ex` match expression is in a local (`let`) or assign expression"] fn opt_parent_assign_span < 'a > (cx : & LateContext < 'a > , ex : & Expr < 'a >) -> Option < AssignmentExpr > { if let Node :: Expr (parent_arm_expr) = cx . tcx . parent_hir_node (ex . hir_id) { return match cx . tcx . parent_hir_node (parent_arm_expr . hir_id) { Node :: LetStmt (parent_let_expr) => Some (AssignmentExpr :: Local { span : parent_let_expr . span , pat_span : parent_let_expr . pat . span () , }) , Node :: Expr (Expr { kind : ExprKind :: Assign (parent_assign_expr , match_expr , _) , .. }) => Some (AssignmentExpr :: Assign { span : parent_assign_expr . span , match_span : match_expr . span , }) , _ => None , } ; } None }
};
}
