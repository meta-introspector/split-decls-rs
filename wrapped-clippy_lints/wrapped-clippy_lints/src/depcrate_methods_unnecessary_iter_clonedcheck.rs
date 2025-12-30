// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_iter_clonedcheck {
() => {
// Module: crate::methods::unnecessary_iter_cloned
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , method_name : Symbol , receiver : & Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , expr) && let Some (callee_def_id) = fn_def_id (cx , parent) && is_into_iter (cx , callee_def_id) { check_for_loop_iter (cx , parent , method_name , receiver , false) } else { false } }
};
}
