// Generated macro for impl_517 (impl)
macro_rules! Depcrate_ffi_taskimpl_517 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_517"}
// Dependencies: {}
impl < T > IntoDynTaskType for T where T : AsTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { Box :: new (self) } }
};
}
