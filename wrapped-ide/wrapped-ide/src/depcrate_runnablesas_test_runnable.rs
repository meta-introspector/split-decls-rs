// Generated macro for as_test_runnable (function)
macro_rules! Depcrate_runnablesas_test_runnable {
() => {
// Module: crate::runnables
// Provides: {"as_test_runnable"}
// Dependencies: {}
fn as_test_runnable (sema : & Semantics < '_ , RootDatabase > , fn_def : & ast :: Fn) -> Option < Runnable > { if test_related_attribute_syn (fn_def) . is_some () { let function = sema . to_def (fn_def) ? ; runnable_fn (sema , function) } else { None } }
};
}
