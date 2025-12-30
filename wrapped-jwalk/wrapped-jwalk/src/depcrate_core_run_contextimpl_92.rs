// Generated macro for impl_92 (impl)
macro_rules! Depcrate_core_run_contextimpl_92 {
() => {
// Module: crate::core::run_context
// Provides: {"impl_92"}
// Dependencies: {}
impl < C : ClientState > Clone for RunContext < C > { fn clone (& self) -> Self { RunContext { stop : self . stop . clone () , read_dir_spec_queue : self . read_dir_spec_queue . clone () , read_dir_result_queue : self . read_dir_result_queue . clone () , core_read_dir_callback : self . core_read_dir_callback . clone () , } } }
};
}
