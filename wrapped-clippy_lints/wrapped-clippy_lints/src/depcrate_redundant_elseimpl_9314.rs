// Generated macro for impl_9314 (impl)
macro_rules! Depcrate_redundant_elseimpl_9314 {
() => {
// Module: crate::redundant_else
// Provides: {"impl_9314"}
// Dependencies: {}
impl EarlyLintPass for RedundantElse { fn check_stmt (& mut self , cx : & EarlyContext < '_ > , stmt : & Stmt) { if stmt . span . in_external_macro (cx . sess () . source_map ()) { return ; } let expr : & Expr = match & stmt . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => expr , _ => return , } ; let (mut then , mut els) : (& Block , & Expr) = match & expr . kind { ExprKind :: If (_ , then , Some (els)) => (then , els) , _ => return , } ; loop { if ! BreakVisitor :: default () . check_block (then) { return ; } match & els . kind { ExprKind :: If (_ , next_then , Some (next_els)) => { then = next_then ; els = next_els ; } , ExprKind :: If (..) => return , _ => break , } } let mut app = Applicability :: MachineApplicable ; if let ExprKind :: Block (block , _) = & els . kind { for stmt in & block . stmts { if matches ! (& stmt . kind , StmtKind :: Let (_) | StmtKind :: MacCall (_)) { app = Applicability :: Unspecified ; break ; } } } span_lint_and_sugg (cx , REDUNDANT_ELSE , els . span . with_lo (then . span . hi ()) , "redundant else block" , "remove the `else` block and move the contents out" , make_sugg (cx , els . span , ".." , Some (expr . span)) , app ,) ; } }
};
}
