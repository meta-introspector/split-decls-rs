// Generated macro for check (function)
macro_rules! Depcrate_loops_while_let_loopcheck {
() => {
// Module: crate::loops::while_let_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , loop_block : & 'tcx Block < '_ >) { let (init , let_info , els) = match (loop_block . stmts , loop_block . expr) { ([stmt , ..] , _) => match stmt . kind { StmtKind :: Let (LetStmt { init : Some (e) , els , pat , ty , .. }) => (* e , Some ((* pat , * ty)) , * els) , StmtKind :: Semi (e) | StmtKind :: Expr (e) => (e , None , None) , _ => return , } , ([] , Some (e)) => (e , None , None) , _ => return , } ; let has_trailing_exprs = loop_block . stmts . len () + usize :: from (loop_block . expr . is_some ()) > 1 ; if let Some (if_let) = higher :: IfLet :: hir (cx , init) && let Some (else_expr) = if_let . if_else && is_simple_break_expr (else_expr) { could_be_while_let (cx , expr , if_let . let_pat , if_let . let_expr , has_trailing_exprs , let_info , Some (if_let . if_then) ,) ; } else if els . and_then (| x | x . expr) . is_some_and (is_simple_break_expr) && let Some ((pat , _)) = let_info { could_be_while_let (cx , expr , pat , init , has_trailing_exprs , let_info , None) ; } else if let ExprKind :: Match (scrutinee , [arm1 , arm2] , MatchSource :: Normal) = init . kind && arm1 . guard . is_none () && arm2 . guard . is_none () && is_simple_break_expr (arm2 . body) { could_be_while_let (cx , expr , arm1 . pat , scrutinee , has_trailing_exprs , let_info , Some (arm1 . body) ,) ; } }
};
}
