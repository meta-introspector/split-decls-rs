// Generated macro for check_correctness (function)
macro_rules! Depcratecheck_correctness {
() => {
// Module: crate
// Provides: {"check_correctness"}
// Dependencies: {}
fn check_correctness () { assert ! (ENTERED_ENTRY_POINT . with (Cell :: get) != 0 , "proc-macro-error2 API cannot be used outside of `entry_point` invocation, \
             perhaps you forgot to annotate your #[proc_macro] function with `#[proc_macro_error]") ; }
};
}
