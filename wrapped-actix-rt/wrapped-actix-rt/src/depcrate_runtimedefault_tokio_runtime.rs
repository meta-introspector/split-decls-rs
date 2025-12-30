// Generated macro for default_tokio_runtime (function)
macro_rules! Depcrate_runtimedefault_tokio_runtime {
() => {
// Module: crate::runtime
// Provides: {"default_tokio_runtime"}
// Dependencies: {}
pub (crate) fn default_tokio_runtime () -> io :: Result < tokio :: runtime :: Runtime > { tokio :: runtime :: Builder :: new_current_thread () . enable_io () . enable_time () . build () }
};
}
