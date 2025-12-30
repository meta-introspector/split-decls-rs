// Generated macro for contains_todo_unimplement_macro (function)
macro_rules! Depcrate_usagecontains_todo_unimplement_macro {
() => {
// Module: crate::usage
// Provides: {"contains_todo_unimplement_macro"}
// Dependencies: {}
# [doc = " Checks if the given expression contains macro call to `todo!()` or `unimplemented!()`."] pub fn contains_todo_unimplement_macro (cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) -> bool { for_each_expr_without_closures (expr , | e | { if is_todo_unimplemented_macro (cx , e) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
