// Generated macro for peer_tmp_key_p384 (function)
macro_rules! Depcrate_ssl_testpeer_tmp_key_p384 {
() => {
// Module: crate::ssl::test
// Provides: {"peer_tmp_key_p384"}
// Dependencies: {}
# [test] # [cfg (ossl300)] fn peer_tmp_key_p384 () { let mut server = Server :: builder () ; server . ctx () . set_groups_list ("P-384") . unwrap () ; let server = server . build () ; let s = server . client () . connect () ; let peer_temp = s . ssl () . peer_tmp_key () . unwrap () ; assert_eq ! (peer_temp . id () , Id :: EC) ; assert_eq ! (peer_temp . bits () , 384) ; let local_temp = s . ssl () . tmp_key () . unwrap () ; assert_eq ! (local_temp . id () , Id :: EC) ; assert_eq ! (local_temp . bits () , 384) ; assert_ne ! (peer_temp . ec_key () . unwrap () . public_key_to_der () . unwrap () , local_temp . ec_key () . unwrap () . public_key_to_der () . unwrap () ,) ; }
};
}
