// Generated macro for impl_284 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelimpl_284 {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"impl_284"}
// Dependencies: {}
impl < C > Tunnel < C > { # [doc = " Create a new Tunnel service."] # [doc = ""] # [doc = " This wraps an underlying connector, and stores the address of a"] # [doc = " tunneling proxy server."] # [doc = ""] # [doc = " A `Tunnel` can then be called with any destination. The `dst` passed to"] # [doc = " `call` will not be used to create the underlying connection, but will"] # [doc = " be used in an HTTP CONNECT request sent to the proxy destination."] pub fn new (proxy_dst : Uri , connector : C) -> Self { Self { headers : Headers :: Empty , inner : connector , proxy_dst , } } # [doc = " Add `proxy-authorization` header value to the CONNECT request."] pub fn with_auth (mut self , mut auth : HeaderValue) -> Self { auth . set_sensitive (true) ; match self . headers { Headers :: Empty => { self . headers = Headers :: Auth (auth) ; } Headers :: Auth (ref mut existing) => { * existing = auth ; } Headers :: Extra (ref mut extra) => { extra . insert (http :: header :: PROXY_AUTHORIZATION , auth) ; } } self } # [doc = " Add extra headers to be sent with the CONNECT request."] # [doc = ""] # [doc = " If existing headers have been set, these will be merged."] pub fn with_headers (mut self , mut headers : HeaderMap) -> Self { match self . headers { Headers :: Empty => { self . headers = Headers :: Extra (headers) ; } Headers :: Auth (auth) => { headers . entry (http :: header :: PROXY_AUTHORIZATION) . or_insert (auth) ; self . headers = Headers :: Extra (headers) ; } Headers :: Extra (ref mut extra) => { extra . extend (headers) ; } } self } }
};
}
