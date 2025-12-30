// Generated macro for sni_callback_swapped_ctx (function)
macro_rules! Depcrate_ssl_testsni_callback_swapped_ctx {
() => {
// Module: crate::ssl::test
// Provides: {"sni_callback_swapped_ctx"}
// Dependencies: {}
# [test] fn sni_callback_swapped_ctx () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_servername_callback (| _ , _ | { CALLED_BACK . store (true , Ordering :: SeqCst) ; Ok (()) }) ; let keyed_ctx = mem :: replace (server . ctx () , ctx) . build () ; server . ssl_cb (move | ssl | ssl . set_ssl_context (& keyed_ctx) . unwrap ()) ; let server = server . build () ; server . client () . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
