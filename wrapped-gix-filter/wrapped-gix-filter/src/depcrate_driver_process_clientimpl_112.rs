// Generated macro for impl_112 (impl)
macro_rules! Depcrate_driver_process_clientimpl_112 {
() => {
// Module: crate::driver::process::client
// Provides: {"impl_112"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Client { # [doc = " Return the child handle of the running process."] # [doc = ""] # [doc = " Note that this will naturally close input and output handles, which is a signal for the child process to shutdown."] pub fn into_child (self) -> std :: process :: Child { self . child } }
};
}
