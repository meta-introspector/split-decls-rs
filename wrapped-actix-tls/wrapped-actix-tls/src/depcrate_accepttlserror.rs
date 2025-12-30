// Generated macro for TlsError (enum)
macro_rules! Depcrate_acceptTlsError {
() => {
// Module: crate::accept
// Provides: {"TlsError"}
// Dependencies: {}
# [doc = " TLS handshake error, TLS timeout, or inner service error."] # [doc = ""] # [doc = " All TLS acceptors from this crate will return the `SvcErr` type parameter as [`Infallible`],"] # [doc = " which can be cast to your own service type, inferred or otherwise, using [`into_service_error`]."] # [doc = ""] # [doc = " [`into_service_error`]: Self::into_service_error"] # [derive (Debug)] pub enum TlsError < TlsErr , SvcErr > { # [doc = " TLS handshake has timed-out."] Timeout , # [doc = " Wraps TLS service errors."] Tls (TlsErr) , # [doc = " Wraps service errors."] Service (SvcErr) , }
};
}
