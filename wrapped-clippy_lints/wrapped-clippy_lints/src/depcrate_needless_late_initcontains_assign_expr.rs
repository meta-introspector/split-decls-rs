// Generated macro for contains_assign_expr (function)
macro_rules! Depcrate_needless_late_initcontains_assign_expr {
() => {
// Module: crate::needless_late_init
// Provides: {"contains_assign_expr"}
// Dependencies: {}
fn contains_assign_expr < 'tcx > (cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < 'tcx >) -> bool { for_each_expr (cx , stmt , | e | { if matches ! (e . kind , ExprKind :: Assign (..)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
