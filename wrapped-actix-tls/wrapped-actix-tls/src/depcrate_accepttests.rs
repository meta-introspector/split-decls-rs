// Generated macro for tests (module)
macro_rules! Depcrate_accepttests {
() => {
// Module: crate::accept
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn tls_service_error_inference () { let a : TlsError < u32 , Infallible > = TlsError :: Tls (42) ; let _b : TlsError < u32 , u64 > = a . into_service_error () ; } }
};
}
