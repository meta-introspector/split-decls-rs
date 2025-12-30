// Generated macro for ShutdownResult (enum)
macro_rules! Depcrate_sslShutdownResult {
() => {
// Module: crate::ssl
// Provides: {"ShutdownResult"}
// Dependencies: {}
# [doc = " The result of a shutdown request."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ShutdownResult { # [doc = " A close notify message has been sent to the peer."] Sent , # [doc = " A close notify response message has been received from the peer."] Received , }
};
}
