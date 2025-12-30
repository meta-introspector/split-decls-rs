// Generated macro for impl_57 (impl)
macro_rules! Depcrate_optimizerimpl_57 {
() => {
// Module: crate::optimizer
// Provides: {"impl_57"}
// Dependencies: {}
impl Iterator for OptimizedExprTopDownIterator { type Item = OptimizedExpr ; fn next (& mut self) -> Option < Self :: Item > { let result = self . current . take () ; if let Some (expr) = self . next . take () { self . iterate_expr (expr) ; } else if let Some (expr) = self . right_branches . pop () { self . iterate_expr (expr) ; } result } }
};
}
