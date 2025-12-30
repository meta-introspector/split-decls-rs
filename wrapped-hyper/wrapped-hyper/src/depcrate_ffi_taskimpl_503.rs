// Generated macro for impl_503 (impl)
macro_rules! Depcrate_ffi_taskimpl_503 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_503"}
// Dependencies: {}
impl < F > crate :: rt :: Executor < F > for WeakExec where F : Future + Send + 'static , F :: Output : Send + Sync + AsTaskType , { fn execute (& self , fut : F) { if let Some (exec) = self . 0 . upgrade () { exec . spawn (hyper_task :: boxed (fut)) ; } } }
};
}
