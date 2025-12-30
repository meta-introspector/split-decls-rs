// Generated macro for impl_509 (impl)
macro_rules! Depcrate_ffi_taskimpl_509 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_509"}
// Dependencies: {}
impl Future for TaskFuture { type Output = Box < hyper_task > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match Pin :: new (& mut self . task . as_mut () . unwrap () . future) . poll (cx) { Poll :: Ready (val) => { let mut task = self . task . take () . unwrap () ; task . output = Some (val) ; Poll :: Ready (task) } Poll :: Pending => Poll :: Pending , } } }
};
}
