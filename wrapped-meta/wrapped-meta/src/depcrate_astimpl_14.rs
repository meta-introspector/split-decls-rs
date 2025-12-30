// Generated macro for impl_14 (impl)
macro_rules! Depcrate_astimpl_14 {
() => {
// Module: crate::ast
// Provides: {"impl_14"}
// Dependencies: {}
impl Iterator for ExprTopDownIterator { type Item = Expr ; fn next (& mut self) -> Option < Self :: Item > { let result = self . current . take () ; if let Some (expr) = self . next . take () { self . iterate_expr (expr) ; } else if let Some (expr) = self . right_branches . pop () { self . iterate_expr (expr) ; } result } }
};
}
