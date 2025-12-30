// Generated macro for impl_157 (impl)
macro_rules! Depcrate_acceptimpl_157 {
() => {
// Module: crate::accept
// Provides: {"impl_157"}
// Dependencies: {}
impl < TlsErr > TlsError < TlsErr , Infallible > { # [doc = " Casts the infallible service error type returned from acceptors into caller's type."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::convert::Infallible;"] # [doc = " # use actix_tls::accept::TlsError;"] # [doc = " let a: TlsError<u32, Infallible> = TlsError::Tls(42);"] # [doc = " let _b: TlsError<u32, u64> = a.into_service_error();"] # [doc = " ```"] pub fn into_service_error < SvcErr > (self) -> TlsError < TlsErr , SvcErr > { match self { Self :: Timeout => TlsError :: Timeout , Self :: Tls (err) => TlsError :: Tls (err) , Self :: Service (err) => match err { } , } } }
};
}
