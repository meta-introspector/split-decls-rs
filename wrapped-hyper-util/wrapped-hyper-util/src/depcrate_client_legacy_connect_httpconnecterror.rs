// Generated macro for ConnectError (struct)
macro_rules! Depcrate_client_legacy_connect_httpConnectError {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"ConnectError"}
// Dependencies: {}
pub struct ConnectError { msg : & 'static str , addr : Option < SocketAddr > , cause : Option < Box < dyn StdError + Send + Sync > > , }
};
}
