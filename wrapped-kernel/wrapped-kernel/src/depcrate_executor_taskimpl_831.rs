// Generated macro for impl_831 (impl)
macro_rules! Depcrate_executor_taskimpl_831 {
() => {
// Module: crate::executor::task
// Provides: {"impl_831"}
// Dependencies: {}
impl Future for AsyncTask { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { trace ! ("Run async task {}" , self . id) ; self . as_mut () . future . as_mut () . poll (cx) } }
};
}
