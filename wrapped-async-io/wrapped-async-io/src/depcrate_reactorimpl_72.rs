// Generated macro for impl_72 (impl)
macro_rules! Depcrate_reactorimpl_72 {
() => {
// Module: crate::reactor
// Provides: {"impl_72"}
// Dependencies: {}
impl < T > Future for Writable < '_ , T > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (Pin :: new (& mut self . 0) . poll (cx)) ? ; # [cfg (feature = "tracing")] tracing :: trace ! (fd = ? self . 0 . handle . source . registration , "writable") ; Poll :: Ready (Ok (())) } }
};
}
