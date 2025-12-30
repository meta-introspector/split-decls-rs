// Generated macro for ZeroRttAccepted (struct)
macro_rules! Depcrate_connectionZeroRttAccepted {
() => {
// Module: crate::connection
// Provides: {"ZeroRttAccepted"}
// Dependencies: {}
# [doc = " Future that completes when a connection is fully established"] # [doc = ""] # [doc = " For clients, the resulting value indicates if 0-RTT was accepted. For servers, the resulting"] # [doc = " value is meaningless."] pub struct ZeroRttAccepted (oneshot :: Receiver < bool >) ;
};
}
