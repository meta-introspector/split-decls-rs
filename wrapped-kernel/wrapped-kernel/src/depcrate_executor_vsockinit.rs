// Generated macro for init (function)
macro_rules! Depcrate_executor_vsockinit {
() => {
// Module: crate::executor::vsock
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { info ! ("Try to initialize vsock interface!") ; spawn (vsock_run ()) ; }
};
}
