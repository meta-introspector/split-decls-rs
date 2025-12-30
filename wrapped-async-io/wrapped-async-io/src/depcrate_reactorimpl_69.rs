// Generated macro for impl_69 (impl)
macro_rules! Depcrate_reactorimpl_69 {
() => {
// Module: crate::reactor
// Provides: {"impl_69"}
// Dependencies: {}
impl < T > Future for ReadableOwned < T > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (Pin :: new (& mut self . 0) . poll (cx)) ? ; # [cfg (feature = "tracing")] tracing :: trace ! (fd = ? self . 0 . handle . source . registration , "readable_owned") ; Poll :: Ready (Ok (())) } }
};
}
