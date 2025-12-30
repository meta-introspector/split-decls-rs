// Generated macro for is_thread_local (function)
macro_rules! Depcrate_non_copy_constis_thread_local {
() => {
// Module: crate::non_copy_const
// Provides: {"is_thread_local"}
// Dependencies: {}
fn is_thread_local (cx : & LateContext < '_ > , it : & Item < '_ >) -> bool { macro_backtrace (it . span) . any (| macro_call | { matches ! (cx . tcx . get_diagnostic_name (macro_call . def_id) , Some (sym :: thread_local_macro)) }) }
};
}
