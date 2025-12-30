// Generated macro for ssl_ex_data_leak (function)
macro_rules! Depcrate_ssl_testssl_ex_data_leak {
() => {
// Module: crate::ssl::test
// Provides: {"ssl_ex_data_leak"}
// Dependencies: {}
# [test] fn ssl_ex_data_leak () { static DROPS : AtomicUsize = AtomicUsize :: new (0) ; struct DropTest ; impl Drop for DropTest { fn drop (& mut self) { DROPS . fetch_add (1 , Ordering :: Relaxed) ; } } let idx = Ssl :: new_ex_index () . unwrap () ; let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () . build () ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl . set_ex_data (idx , DropTest) ; ssl . set_ex_data (idx , DropTest) ; assert_eq ! (DROPS . load (Ordering :: Relaxed) , 1) ; drop (ssl) ; assert_eq ! (DROPS . load (Ordering :: Relaxed) , 2) ; }
};
}
