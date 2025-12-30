// Generated macro for TLS (static)
macro_rules! Depcrate_capiTLS {
() => {
// Module: crate::capi
// Provides: {"TLS"}
// Dependencies: {}
# [thread_local] static TLS : UnsafeCell < * mut u8 > = UnsafeCell :: new (core :: ptr :: null_mut ()) ;
};
}
