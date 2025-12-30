// Generated macro for impl_66 (impl)
macro_rules! Depcrate_reactorimpl_66 {
() => {
// Module: crate::reactor
// Provides: {"impl_66"}
// Dependencies: {}
impl < T > Future for Readable < '_ , T > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (Pin :: new (& mut self . 0) . poll (cx)) ? ; # [cfg (feature = "tracing")] tracing :: trace ! (fd = ? self . 0 . handle . source . registration , "readable") ; Poll :: Ready (Ok (())) } }
};
}
