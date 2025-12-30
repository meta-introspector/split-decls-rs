// Generated macro for impl_7635 (impl)
macro_rules! Depcrate_multi_assignmentsimpl_7635 {
() => {
// Module: crate::multi_assignments
// Provides: {"impl_7635"}
// Dependencies: {}
impl EarlyLintPass for MultiAssignments { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Assign (target , source , _) = & expr . kind { if let ExprKind :: Assign (_target , _source , _) = & strip_paren_blocks (target) . kind { span_lint (cx , MULTI_ASSIGNMENTS , expr . span , "assignments don't nest intuitively") ; } if let ExprKind :: Assign (_target , _source , _) = & strip_paren_blocks (source) . kind { span_lint (cx , MULTI_ASSIGNMENTS , expr . span , "assignments don't nest intuitively") ; } } } }
};
}
