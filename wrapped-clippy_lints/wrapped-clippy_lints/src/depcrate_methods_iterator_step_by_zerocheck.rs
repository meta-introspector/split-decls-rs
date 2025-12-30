// Generated macro for check (function)
macro_rules! Depcrate_methods_iterator_step_by_zerocheck {
() => {
// Module: crate::methods::iterator_step_by_zero
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ > , arg : & 'tcx hir :: Expr < '_ >) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let Some (Constant :: Int (0)) = ConstEvalCtxt :: new (cx) . eval (arg) { span_lint (cx , ITERATOR_STEP_BY_ZERO , expr . span , "`Iterator::step_by(0)` will panic at runtime" ,) ; } }
};
}
