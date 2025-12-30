// Generated macro for impl_139 (impl)
macro_rules! Depcrate_future_future_remote_handleimpl_139 {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"impl_139"}
// Dependencies: {}
impl < T : 'static > Future for RemoteHandle < T > { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { match ready ! (self . rx . poll_unpin (cx)) { Ok (Ok (output)) => Poll :: Ready (output) , Ok (Err (e)) => panic :: resume_unwind (e) , Err (e) => panic :: resume_unwind (Box :: new (e)) , } } }
};
}
