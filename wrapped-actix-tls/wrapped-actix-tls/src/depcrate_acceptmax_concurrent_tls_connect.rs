// Generated macro for max_concurrent_tls_connect (function)
macro_rules! Depcrate_acceptmax_concurrent_tls_connect {
() => {
// Module: crate::accept
// Provides: {"max_concurrent_tls_connect"}
// Dependencies: {}
# [doc = " Sets the maximum per-worker concurrent TLS connection limit."] # [doc = ""] # [doc = " All listeners will stop accepting connections when this limit is reached."] # [doc = " It can be used to regulate the global TLS CPU usage."] # [doc = ""] # [doc = " By default, the connection limit is 256."] pub fn max_concurrent_tls_connect (num : usize) { MAX_CONN . store (num , Ordering :: Relaxed) ; }
};
}
