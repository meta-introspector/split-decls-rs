// Generated macro for impl_519 (impl)
macro_rules! Depcrate_ffi_taskimpl_519 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_519"}
// Dependencies: {}
impl < T > IntoDynTaskType for Option < T > where T : IntoDynTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { match self { Some (val) => val . into_dyn_task_type () , None => () . into_dyn_task_type () , } } }
};
}
