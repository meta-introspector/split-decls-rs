// Generated macro for impl_1454 (impl)
macro_rules! Depcrate_utils_execimpl_1454 {
() => {
// Module: crate::utils::exec
// Provides: {"impl_1454"}
// Dependencies: {}
impl StreamingCommand { pub fn wait (mut self , exec_ctx : impl AsRef < ExecutionContext > ,) -> Result < ExitStatus , std :: io :: Error > { let exec_ctx = exec_ctx . as_ref () ; let output = self . child . wait () ; exec_ctx . profiler () . record_execution (self . fingerprint , self . start_time) ; output } }
};
}
