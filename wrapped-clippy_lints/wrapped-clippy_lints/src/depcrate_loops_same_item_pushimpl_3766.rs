// Generated macro for impl_3766 (impl)
macro_rules! Depcrate_loops_same_item_pushimpl_3766 {
() => {
// Module: crate::loops::same_item_push
// Provides: {"impl_3766"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SameItemPushVisitor < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { match & expr . kind { ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: If (..) => self . non_deterministic_expr = true , ExprKind :: Block (block , _) => self . visit_block (block) , _ => { if let Some (hir_id) = path_to_local (expr) { self . used_locals . insert (hir_id) ; } walk_expr (self , expr) ; } , } } fn visit_block (& mut self , b : & 'tcx Block < '_ >) { for stmt in b . stmts { self . visit_stmt (stmt) ; } } fn visit_stmt (& mut self , s : & 'tcx Stmt < '_ >) { let vec_push_option = get_vec_push (self . cx , s) ; if vec_push_option . is_none () { match & s . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => self . visit_expr (expr) , _ => { } , } } else if self . vec_push . is_none () { self . vec_push = vec_push_option ; } else { self . multiple_pushes = true ; } } }
};
}
