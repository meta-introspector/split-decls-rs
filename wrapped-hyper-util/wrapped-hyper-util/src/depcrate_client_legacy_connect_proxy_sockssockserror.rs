// Generated macro for SocksError (enum)
macro_rules! Depcrate_client_legacy_connect_proxy_socksSocksError {
() => {
// Module: crate::client::legacy::connect::proxy::socks
// Provides: {"SocksError"}
// Dependencies: {}
# [derive (Debug)] pub enum SocksError < C > { Inner (C) , Io (std :: io :: Error) , DnsFailure , MissingHost , MissingPort , V4 (SocksV4Error) , V5 (SocksV5Error) , Parsing (ParsingError) , Serialize (SerializeError) , }
};
}
