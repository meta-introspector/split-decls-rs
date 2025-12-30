// Generated macro for TrySendError (enum)
macro_rules! Depcrate_client_legacy_clientTrySendError {
() => {
// Module: crate::client::legacy::client
// Provides: {"TrySendError"}
// Dependencies: {}
enum TrySendError < B > { Retryable { error : Error , req : Request < B > , connection_reused : bool , } , Nope (Error) , }
};
}
