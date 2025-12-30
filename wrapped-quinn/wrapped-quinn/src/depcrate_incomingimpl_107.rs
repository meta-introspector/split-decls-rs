// Generated macro for impl_107 (impl)
macro_rules! Depcrate_incomingimpl_107 {
() => {
// Module: crate::incoming
// Provides: {"impl_107"}
// Dependencies: {}
impl IntoFuture for Incoming { type Output = Result < Connection , ConnectionError > ; type IntoFuture = IncomingFuture ; fn into_future (self) -> Self :: IntoFuture { IncomingFuture (self . accept ()) } }
};
}
