// Generated macro for impl_87 (impl)
macro_rules! Depcrate_streamimpl_87 {
() => {
// Module: crate::stream
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , F > Stream for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < Option < T > > , { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { (& mut self . f) (cx) } }
};
}
