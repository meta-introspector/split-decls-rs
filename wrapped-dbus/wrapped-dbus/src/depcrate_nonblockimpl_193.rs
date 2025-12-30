// Generated macro for impl_193 (impl)
macro_rules! Depcrate_nonblockimpl_193 {
() => {
// Module: crate::nonblock
// Provides: {"impl_193"}
// Dependencies: {}
impl < T > MethodReply < T > { # [doc = " Creates a new method reply from a future."] fn new < Fut : Future < Output = Result < T , Error > > + Send + 'static > (fut : Fut) -> Self { MethodReply (Box :: pin (fut)) } }
};
}
