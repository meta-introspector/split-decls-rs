// Generated macro for no_version_overlap (function)
macro_rules! Depcrate_ssl_testno_version_overlap {
() => {
// Module: crate::ssl::test
// Provides: {"no_version_overlap"}
// Dependencies: {}
# [test] # [cfg (any (ossl110 , libressl))] fn no_version_overlap () { let mut server = Server :: builder () ; server . ctx () . set_min_proto_version (None) . unwrap () ; server . ctx () . set_max_proto_version (Some (SslVersion :: TLS1_1)) . unwrap () ; # [cfg (any (ossl110g , libressl))] assert_eq ! (server . ctx () . max_proto_version () , Some (SslVersion :: TLS1_1)) ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_min_proto_version (Some (SslVersion :: TLS1_2)) . unwrap () ; # [cfg (ossl110g)] assert_eq ! (client . ctx () . min_proto_version () , Some (SslVersion :: TLS1_2)) ; client . ctx () . set_max_proto_version (None) . unwrap () ; client . connect_err () ; }
};
}
