// Generated macro for impl_2671 (impl)
macro_rules! Depcrate_if_not_elseimpl_2671 {
() => {
// Module: crate::if_not_else
// Provides: {"impl_2671"}
// Dependencies: {}
impl LateLintPass < '_ > for IfNotElse { fn check_expr (& mut self , cx : & LateContext < '_ > , e : & Expr < '_ >) { if let ExprKind :: If (cond , cond_inner , Some (els)) = e . kind && let ExprKind :: Block (..) = els . kind { let (msg , help) = match cond . kind { ExprKind :: Unary (UnOp :: Not , _) => ("unnecessary boolean `not` operation" , "remove the `!` and swap the blocks of the `if`/`else`" ,) , ExprKind :: Binary (op , _ , rhs) if op . node == BinOpKind :: Ne && ! is_zero_integer_const (cx , rhs , e . span . ctxt ()) => { ("unnecessary `!=` operation" , "change to `==` and swap the blocks of the `if`/`else`" ,) } , _ => return , } ; if ! e . span . from_expansion () && ! is_else_clause (cx . tcx , e) { match cond . kind { ExprKind :: Unary (UnOp :: Not , _) | ExprKind :: Binary (_ , _ , _) => span_lint_and_sugg (cx , IF_NOT_ELSE , e . span , msg , "try" , make_sugg (cx , & cond . kind , cond_inner . span , els . span , ".." , Some (e . span)) , Applicability :: MachineApplicable ,) , _ => span_lint_and_help (cx , IF_NOT_ELSE , e . span , msg , None , help) , } } } } }
};
}
