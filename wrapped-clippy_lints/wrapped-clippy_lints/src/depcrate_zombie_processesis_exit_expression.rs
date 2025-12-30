// Generated macro for is_exit_expression (function)
macro_rules! Depcrate_zombie_processesis_exit_expression {
() => {
// Module: crate::zombie_processes
// Provides: {"is_exit_expression"}
// Dependencies: {}
# [doc = " Checks if the given expression exits the process."] fn is_exit_expression (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (fn_did) = fn_def_id (cx , expr) && let Some (fn_name) = cx . tcx . get_diagnostic_name (fn_did) && matches ! (fn_name , sym :: process_exit | sym :: process_abort) { true } else { false } }
};
}
