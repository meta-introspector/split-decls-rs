// Generated macro for export_keying_material (function)
macro_rules! Depcrate_testsexport_keying_material {
() => {
// Module: crate::tests
// Provides: {"export_keying_material"}
// Dependencies: {}
# [test] fn export_keying_material () { let _guard = subscribe () ; let runtime = rt_basic () ; let endpoint = { let _guard = runtime . enter () ; endpoint () } ; runtime . block_on (async move { let outgoing_conn_fut = tokio :: spawn ({ let endpoint = endpoint . clone () ; async move { endpoint . connect (endpoint . local_addr () . unwrap () , "localhost") . unwrap () . await . expect ("connect") } }) ; let incoming_conn_fut = tokio :: spawn ({ let endpoint = endpoint . clone () ; async move { endpoint . accept () . await . expect ("endpoint") . await . expect ("connection") } }) ; let outgoing_conn = outgoing_conn_fut . await . unwrap () ; let incoming_conn = incoming_conn_fut . await . unwrap () ; let mut i_buf = [0u8 ; 64] ; incoming_conn . export_keying_material (& mut i_buf , b"asdf" , b"qwer") . unwrap () ; let mut o_buf = [0u8 ; 64] ; outgoing_conn . export_keying_material (& mut o_buf , b"asdf" , b"qwer") . unwrap () ; assert_eq ! (& i_buf [..] , & o_buf [..]) ; }) ; }
};
}
