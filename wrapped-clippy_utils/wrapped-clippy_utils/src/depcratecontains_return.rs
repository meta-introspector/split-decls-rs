// Generated macro for contains_return (function)
macro_rules! Depcratecontains_return {
() => {
// Module: crate
// Provides: {"contains_return"}
// Dependencies: {}
# [doc = " Returns `true` if `expr` contains a return expression"] pub fn contains_return < 'tcx > (expr : impl Visitable < 'tcx >) -> bool { for_each_expr_without_closures (expr , | e | { if matches ! (e . kind , ExprKind :: Ret (..)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
