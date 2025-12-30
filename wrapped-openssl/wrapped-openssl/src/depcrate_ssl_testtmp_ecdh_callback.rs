// Generated macro for tmp_ecdh_callback (function)
macro_rules! Depcrate_ssl_testtmp_ecdh_callback {
() => {
// Module: crate::ssl::test
// Provides: {"tmp_ecdh_callback"}
// Dependencies: {}
# [test] # [cfg (all (ossl102 , not (ossl110)))] # [allow (deprecated)] fn tmp_ecdh_callback () { use crate :: ec :: EcKey ; use crate :: nid :: Nid ; static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_tmp_ecdh_callback (| _ , _ , _ | { CALLED_BACK . store (true , Ordering :: SeqCst) ; EcKey :: from_curve_name (Nid :: X9_62_PRIME256V1) }) ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_cipher_list ("ECDH") . unwrap () ; client . connect () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
