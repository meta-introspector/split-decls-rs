// Generated macro for BoxConnecting (type)
macro_rules! Depcrate_client_legacy_connect_httpBoxConnecting {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"BoxConnecting"}
// Dependencies: {}
type BoxConnecting = Pin < Box < dyn Future < Output = ConnectResult > + Send > > ;
};
}
