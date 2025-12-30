// Generated macro for tmp_dh_callback (function)
macro_rules! Depcrate_ssl_testtmp_dh_callback {
() => {
// Module: crate::ssl::test
// Provides: {"tmp_dh_callback"}
// Dependencies: {}
# [test] # [cfg_attr (any (boringssl , awslc) , ignore)] fn tmp_dh_callback () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_tmp_dh_callback (| _ , _ , _ | { CALLED_BACK . store (true , Ordering :: SeqCst) ; let dh = include_bytes ! ("../../../test/dhparams.pem") ; Dh :: params_from_pem (dh) }) ; let server = server . build () ; let mut client = server . client () ; # [cfg (any (ossl111 , libressl))] client . ctx () . set_options (super :: SslOptions :: NO_TLSV1_3) ; client . ctx () . set_cipher_list ("EDH") . unwrap () ; client . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
