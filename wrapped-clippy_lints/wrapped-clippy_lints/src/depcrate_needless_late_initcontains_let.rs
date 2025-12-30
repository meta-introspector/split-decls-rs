// Generated macro for contains_let (function)
macro_rules! Depcrate_needless_late_initcontains_let {
() => {
// Module: crate::needless_late_init
// Provides: {"contains_let"}
// Dependencies: {}
fn contains_let (cond : & Expr < '_ >) -> bool { for_each_expr_without_closures (cond , | e | { if matches ! (e . kind , ExprKind :: Let (_)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
