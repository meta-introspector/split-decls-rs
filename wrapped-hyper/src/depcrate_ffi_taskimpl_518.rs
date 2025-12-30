// Generated macro for impl_518 (impl)
macro_rules! Depcrate_ffi_taskimpl_518 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_518"}
// Dependencies: {}
impl < T > IntoDynTaskType for crate :: Result < T > where T : IntoDynTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { match self { Ok (val) => val . into_dyn_task_type () , Err (err) => Box :: new (err) , } } }
};
}
