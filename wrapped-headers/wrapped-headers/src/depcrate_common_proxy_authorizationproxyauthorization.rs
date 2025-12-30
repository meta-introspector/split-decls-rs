// Generated macro for ProxyAuthorization (struct)
macro_rules! Depcrate_common_proxy_authorizationProxyAuthorization {
() => {
// Module: crate::common::proxy_authorization
// Provides: {"ProxyAuthorization"}
// Dependencies: {}
# [doc = " `Proxy-Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.4)"] # [doc = ""] # [doc = " The `Proxy-Authorization` header field allows a user agent to authenticate"] # [doc = " itself with an HTTP proxy -- usually, but not necessarily, after"] # [doc = " receiving a 407 (Proxy Authentication Required) response and the"] # [doc = " `Proxy-Authenticate` header. Its value consists of credentials containing"] # [doc = " the authentication information of the user agent for the realm of the"] # [doc = " resource being requested."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Proxy-Authorization = credentials"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`"] # [doc = " * `Bearer fpKL54jvWmEGVoRdCNjG`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [derive (Clone , PartialEq , Debug)] pub struct ProxyAuthorization < C : Credentials > (pub C) ;
};
}
