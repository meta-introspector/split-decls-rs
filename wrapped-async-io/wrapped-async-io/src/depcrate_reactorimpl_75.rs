// Generated macro for impl_75 (impl)
macro_rules! Depcrate_reactorimpl_75 {
() => {
// Module: crate::reactor
// Provides: {"impl_75"}
// Dependencies: {}
impl < T > Future for WritableOwned < T > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (Pin :: new (& mut self . 0) . poll (cx)) ? ; # [cfg (feature = "tracing")] tracing :: trace ! (fd = ? self . 0 . handle . source . registration , "writable_owned") ; Poll :: Ready (Ok (())) } }
};
}
