// Generated macro for ClientHelloResponse (struct)
macro_rules! Depcrate_sslClientHelloResponse {
() => {
// Module: crate::ssl
// Provides: {"ClientHelloResponse"}
// Dependencies: {}
# [doc = " The result of a client hello callback."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [cfg (ossl111)] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct ClientHelloResponse (c_int) ;
};
}
