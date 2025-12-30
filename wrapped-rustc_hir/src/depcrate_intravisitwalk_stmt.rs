// Generated macro for walk_stmt (function)
macro_rules! Depcrate_intravisitwalk_stmt {
() => {
// Module: crate::intravisit
// Provides: {"walk_stmt"}
// Dependencies: {}
pub fn walk_stmt < 'v , V : Visitor < 'v > > (visitor : & mut V , statement : & 'v Stmt < 'v >) -> V :: Result { let Stmt { kind , hir_id , span : _ } = statement ; try_visit ! (visitor . visit_id (* hir_id)) ; match * kind { StmtKind :: Let (ref local) => visitor . visit_local (local) , StmtKind :: Item (item) => visitor . visit_nested_item (item) , StmtKind :: Expr (ref expression) | StmtKind :: Semi (ref expression) => { visitor . visit_expr (expression) } } }
};
}
