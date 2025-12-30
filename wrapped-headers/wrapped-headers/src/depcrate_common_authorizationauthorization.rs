// Generated macro for Authorization (struct)
macro_rules! Depcrate_common_authorizationAuthorization {
() => {
// Module: crate::common::authorization
// Provides: {"Authorization"}
// Dependencies: {}
# [doc = " `Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.2)"] # [doc = ""] # [doc = " The `Authorization` header field allows a user agent to authenticate"] # [doc = " itself with an origin server -- usually, but not necessarily, after"] # [doc = " receiving a 401 (Unauthorized) response.  Its value consists of"] # [doc = " credentials containing the authentication information of the user"] # [doc = " agent for the realm of the resource being requested."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Authorization = credentials"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`"] # [doc = " * `Bearer fpKL54jvWmEGVoRdCNjG`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Authorization;"] # [doc = ""] # [doc = " let basic = Authorization::basic(\"Aladdin\", \"open sesame\");"] # [doc = " let bearer = Authorization::bearer(\"some-opaque-token\").unwrap();"] # [doc = " ```"] # [doc = ""] # [derive (Clone , PartialEq , Debug)] pub struct Authorization < C : Credentials > (pub C) ;
};
}
