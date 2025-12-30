// Generated macro for StopSignal (enum)
macro_rules! Depcrate_signalsStopSignal {
() => {
// Module: crate::signals
// Provides: {"StopSignal"}
// Dependencies: {}
pub (crate) enum StopSignal { # [doc = " OS signal handling is configured."] Os (OsSignals) , # [doc = " Cancellation token or channel."] Cancel (BoxFuture < 'static , () >) , }
};
}
