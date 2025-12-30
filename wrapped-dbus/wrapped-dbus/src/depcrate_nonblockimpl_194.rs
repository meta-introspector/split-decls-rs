// Generated macro for impl_194 (impl)
macro_rules! Depcrate_nonblockimpl_194 {
() => {
// Module: crate::nonblock
// Provides: {"impl_194"}
// Dependencies: {}
impl < T > Future for MethodReply < T > { type Output = Result < T , Error > ; fn poll (mut self : pin :: Pin < & mut Self > , ctx : & mut task :: Context) -> task :: Poll < Result < T , Error > > { self . 0 . as_mut () . poll (ctx) } }
};
}
