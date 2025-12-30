// Generated macro for PoolTx (enum)
macro_rules! Depcrate_client_legacy_clientPoolTx {
() => {
// Module: crate::client::legacy::client
// Provides: {"PoolTx"}
// Dependencies: {}
enum PoolTx < B > { # [cfg (feature = "http1")] Http1 (hyper :: client :: conn :: http1 :: SendRequest < B >) , # [cfg (feature = "http2")] Http2 (hyper :: client :: conn :: http2 :: SendRequest < B >) , }
};
}
