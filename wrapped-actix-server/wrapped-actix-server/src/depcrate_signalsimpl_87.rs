// Generated macro for impl_87 (impl)
macro_rules! Depcrate_signalsimpl_87 {
() => {
// Module: crate::signals
// Provides: {"impl_87"}
// Dependencies: {}
impl Future for OsSignals { type Output = SignalKind ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { # [cfg (not (unix))] { self . signals . as_mut () . poll (cx) . map (| _ | SignalKind :: OsInt) } # [cfg (unix)] { for (sig , fut) in self . signals . iter_mut () { if fut . poll_recv (cx) . is_ready () { trace ! ("{} received" , sig) ; return Poll :: Ready (* sig) ; } } Poll :: Pending } } }
};
}
