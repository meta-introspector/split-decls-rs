// Generated macro for ssl_ctx_ex_data_leak (function)
macro_rules! Depcrate_ssl_testssl_ctx_ex_data_leak {
() => {
// Module: crate::ssl::test
// Provides: {"ssl_ctx_ex_data_leak"}
// Dependencies: {}
# [test] fn ssl_ctx_ex_data_leak () { static DROPS : AtomicUsize = AtomicUsize :: new (0) ; struct DropTest ; impl Drop for DropTest { fn drop (& mut self) { DROPS . fetch_add (1 , Ordering :: Relaxed) ; } } let idx = SslContext :: new_ex_index () . unwrap () ; let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_ex_data (idx , DropTest) ; ctx . set_ex_data (idx , DropTest) ; assert_eq ! (DROPS . load (Ordering :: Relaxed) , 1) ; drop (ctx) ; assert_eq ! (DROPS . load (Ordering :: Relaxed) , 2) ; }
};
}
