// Generated macro for impl_400 (impl)
macro_rules! Depcrate_stream_implsimpl_400 {
() => {
// Module: crate::stream::impls
// Provides: {"impl_400"}
// Dependencies: {}
impl < S : ? Sized + Stream > Stream for Box < S > { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < Self :: Item > , Self :: Error > { (* * self) . poll (task) } fn schedule (& mut self , task : & mut Task) { (* * self) . schedule (task) } }
};
}
