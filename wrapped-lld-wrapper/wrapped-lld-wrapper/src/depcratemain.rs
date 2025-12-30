// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let current_exe_path = env :: current_exe () . unwrap_or_exit_with ("could not get the path of the current executable") ; exec_lld (get_rust_lld_command (current_exe_path . as_ref ())) ; }
};
}
