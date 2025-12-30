// Generated macro for impl_130 (impl)
macro_rules! Depcrate_taskimpl_130 {
() => {
// Module: crate::task
// Provides: {"impl_130"}
// Dependencies: {}
impl < T , M > Future for Task < T , M > { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match poll_task :: < T > (self . ptr . as_ptr () , cx) { Poll :: Ready (t) => Poll :: Ready (t . expect ("Task polled after completion")) , Poll :: Pending => Poll :: Pending , } } }
};
}
