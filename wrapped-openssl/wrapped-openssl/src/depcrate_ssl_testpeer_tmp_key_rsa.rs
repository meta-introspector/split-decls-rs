// Generated macro for peer_tmp_key_rsa (function)
macro_rules! Depcrate_ssl_testpeer_tmp_key_rsa {
() => {
// Module: crate::ssl::test
// Provides: {"peer_tmp_key_rsa"}
// Dependencies: {}
# [test] # [cfg (ossl300)] fn peer_tmp_key_rsa () { let mut server = Server :: builder () ; server . ctx () . set_cipher_list ("RSA") . unwrap () ; server . ctx () . set_max_proto_version (Some (SslVersion :: TLS1_2)) . unwrap () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_groups_list ("P-521") . unwrap () ; let s = client . connect () ; let peer_temp = s . ssl () . peer_tmp_key () ; assert ! (peer_temp . is_err ()) ; let local_temp = s . ssl () . tmp_key () . unwrap () ; assert_eq ! (local_temp . id () , Id :: EC) ; assert_eq ! (local_temp . bits () , 521) ; }
};
}
