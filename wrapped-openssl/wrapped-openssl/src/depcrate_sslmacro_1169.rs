// Generated macro for macro_1169 (macro)
macro_rules! Depcrate_sslmacro_1169 {
() => {
// Module: crate::ssl
// Provides: {"macro_1169"}
// Dependencies: {}
bitflags ! { # [doc = " Options controlling the behavior of session caching."] # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct SslSessionCacheMode : SslBitType { # [doc = " No session caching for the client or server takes place."] const OFF = ffi :: SSL_SESS_CACHE_OFF ; # [doc = " Enable session caching on the client side."] # [doc = ""] # [doc = " OpenSSL has no way of identifying the proper session to reuse automatically, so the"] # [doc = " application is responsible for setting it explicitly via [`SslRef::set_session`]."] # [doc = ""] # [doc = " [`SslRef::set_session`]: struct.SslRef.html#method.set_session"] const CLIENT = ffi :: SSL_SESS_CACHE_CLIENT ; # [doc = " Enable session caching on the server side."] # [doc = ""] # [doc = " This is the default mode."] const SERVER = ffi :: SSL_SESS_CACHE_SERVER ; # [doc = " Enable session caching on both the client and server side."] const BOTH = ffi :: SSL_SESS_CACHE_BOTH ; # [doc = " Disable automatic removal of expired sessions from the session cache."] const NO_AUTO_CLEAR = ffi :: SSL_SESS_CACHE_NO_AUTO_CLEAR ; # [doc = " Disable use of the internal session cache for session lookups."] const NO_INTERNAL_LOOKUP = ffi :: SSL_SESS_CACHE_NO_INTERNAL_LOOKUP ; # [doc = " Disable use of the internal session cache for session storage."] const NO_INTERNAL_STORE = ffi :: SSL_SESS_CACHE_NO_INTERNAL_STORE ; # [doc = " Disable use of the internal session cache for storage and lookup."] const NO_INTERNAL = ffi :: SSL_SESS_CACHE_NO_INTERNAL ; } }
};
}
