// Generated macro for export_keying_material (function)
macro_rules! Depcrate_testsexport_keying_material {
() => {
// Module: crate::tests
// Provides: {"export_keying_material"}
// Dependencies: {}
# [test] fn export_keying_material () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; const LABEL : & [u8] = b"test_label" ; const CONTEXT : & [u8] = b"test_context" ; let mut client_buf = [0u8 ; 64] ; pair . client_conn_mut (client_ch) . crypto_session () . export_keying_material (& mut client_buf , LABEL , CONTEXT) . unwrap () ; let mut server_buf = [0u8 ; 64] ; pair . server_conn_mut (server_ch) . crypto_session () . export_keying_material (& mut server_buf , LABEL , CONTEXT) . unwrap () ; assert_eq ! (& client_buf [..] , & server_buf [..]) ; }
};
}
