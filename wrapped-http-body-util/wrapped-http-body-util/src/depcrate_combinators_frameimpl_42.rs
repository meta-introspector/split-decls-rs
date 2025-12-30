// Generated macro for impl_42 (impl)
macro_rules! Depcrate_combinators_frameimpl_42 {
() => {
// Module: crate::combinators::frame
// Provides: {"impl_42"}
// Dependencies: {}
impl < T : Body + Unpin + ? Sized > Future for Frame < '_ , T > { type Output = Option < Result < http_body :: Frame < T :: Data > , T :: Error > > ; fn poll (mut self : Pin < & mut Self > , ctx : & mut task :: Context < '_ >) -> task :: Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll_frame (ctx) } }
};
}
