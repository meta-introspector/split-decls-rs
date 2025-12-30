// Generated macro for impl_1604 (impl)
macro_rules! Depcrate_stream_poll_fnimpl_1604 {
() => {
// Module: crate::stream::poll_fn
// Provides: {"impl_1604"}
// Dependencies: {}
impl < T , F > Stream for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < Option < T > > , { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { (& mut self . f) (cx) } }
};
}
