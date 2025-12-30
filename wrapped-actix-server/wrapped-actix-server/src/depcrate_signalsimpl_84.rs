// Generated macro for impl_84 (impl)
macro_rules! Depcrate_signalsimpl_84 {
() => {
// Module: crate::signals
// Provides: {"impl_84"}
// Dependencies: {}
impl Future for StopSignal { type Output = SignalKind ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { StopSignal :: Os (os_signals) => pin ! (os_signals) . poll (cx) , StopSignal :: Cancel (cancel) => pin ! (cancel) . poll (cx) . map (| () | SignalKind :: Cancel) , } } }
};
}
