// Generated macro for is_todo_unimplemented_macro (function)
macro_rules! Depcrate_usageis_todo_unimplemented_macro {
() => {
// Module: crate::usage
// Provides: {"is_todo_unimplemented_macro"}
// Dependencies: {}
# [doc = " Checks if the given expression is a macro call to `todo!()` or `unimplemented!()`."] pub fn is_todo_unimplemented_macro (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { root_macro_call_first_node (cx , expr) . and_then (| macro_call | cx . tcx . get_diagnostic_name (macro_call . def_id)) . is_some_and (| macro_name | matches ! (macro_name , sym :: todo_macro | sym :: unimplemented_macro)) }
};
}
