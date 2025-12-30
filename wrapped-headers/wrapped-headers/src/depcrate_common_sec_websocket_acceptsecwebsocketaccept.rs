// Generated macro for SecWebsocketAccept (struct)
macro_rules! Depcrate_common_sec_websocket_acceptSecWebsocketAccept {
() => {
// Module: crate::common::sec_websocket_accept
// Provides: {"SecWebsocketAccept"}
// Dependencies: {}
# [doc = " The `Sec-Websocket-Accept` header."] # [doc = ""] # [doc = " This header is used in the Websocket handshake, sent back by the"] # [doc = " server indicating a successful handshake. It is a signature"] # [doc = " of the `Sec-Websocket-Key` header."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use headers::{SecWebsocketAccept, SecWebsocketKey};"] # [doc = ""] # [doc = " let sec_key: SecWebsocketKey = /* from request headers */"] # [doc = " #    unimplemented!();"] # [doc = ""] # [doc = " let sec_accept = SecWebsocketAccept::from(sec_key);"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct SecWebsocketAccept (HeaderValue) ;
};
}
