// Generated macro for impl_508 (impl)
macro_rules! Depcrate_ffi_taskimpl_508 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_508"}
// Dependencies: {}
impl hyper_task { pub (crate) fn boxed < F > (fut : F) -> Box < hyper_task > where F : Future + Send + 'static , F :: Output : IntoDynTaskType + Send + Sync + 'static , { Box :: new (hyper_task { future : Box :: pin (async move { fut . await . into_dyn_task_type () }) , output : None , userdata : UserDataPointer (ptr :: null_mut ()) , }) } fn output_type (& self) -> hyper_task_return_type { match self . output { None => hyper_task_return_type :: HYPER_TASK_EMPTY , Some (ref val) => val . as_task_type () , } } }
};
}
