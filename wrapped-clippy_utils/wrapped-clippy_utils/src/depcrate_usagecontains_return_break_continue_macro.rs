// Generated macro for contains_return_break_continue_macro (function)
macro_rules! Depcrate_usagecontains_return_break_continue_macro {
() => {
// Module: crate::usage
// Provides: {"contains_return_break_continue_macro"}
// Dependencies: {}
pub fn contains_return_break_continue_macro (expression : & Expr < '_ >) -> bool { for_each_expr_without_closures (expression , | e | { match e . kind { ExprKind :: Ret (..) | ExprKind :: Break (..) | ExprKind :: Continue (..) => ControlFlow :: Break (()) , _ if e . span . from_expansion () => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } }) . is_some () }
};
}
