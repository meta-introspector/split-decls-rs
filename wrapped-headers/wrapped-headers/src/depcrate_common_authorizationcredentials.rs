// Generated macro for Credentials (trait)
macro_rules! Depcrate_common_authorizationCredentials {
() => {
// Module: crate::common::authorization
// Provides: {"Credentials"}
// Dependencies: {}
# [doc = " Credentials to be used in the `Authorization` header."] pub trait Credentials : Sized { # [doc = " The scheme identify the format of these credentials."] # [doc = ""] # [doc = " This is the static string that always prefixes the actual credentials,"] # [doc = " like `\"Basic\"` in basic authorization."] const SCHEME : & 'static str ; # [doc = " Try to decode the credentials from the `HeaderValue`."] # [doc = ""] # [doc = " The `SCHEME` will be the first part of the `value`."] fn decode (value : & HeaderValue) -> Option < Self > ; # [doc = " Encode the credentials to a `HeaderValue`."] # [doc = ""] # [doc = " The `SCHEME` must be the first part of the `value`."] fn encode (& self) -> HeaderValue ; }
};
}
