// Generated macro for contains_break_or_continue (function)
macro_rules! Depcrate_visitorscontains_break_or_continue {
() => {
// Module: crate::visitors
// Provides: {"contains_break_or_continue"}
// Dependencies: {}
pub fn contains_break_or_continue (expr : & Expr < '_ >) -> bool { for_each_expr_without_closures (expr , | e | { if matches ! (e . kind , ExprKind :: Break (..) | ExprKind :: Continue (..)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
