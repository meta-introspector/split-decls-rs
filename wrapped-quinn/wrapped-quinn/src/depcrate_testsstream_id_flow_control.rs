// Generated macro for stream_id_flow_control (function)
macro_rules! Depcrate_testsstream_id_flow_control {
() => {
// Module: crate::tests
// Provides: {"stream_id_flow_control"}
// Dependencies: {}
# [tokio :: test] async fn stream_id_flow_control () { let _guard = subscribe () ; let mut cfg = TransportConfig :: default () ; cfg . max_concurrent_uni_streams (1u32 . into ()) ; let endpoint = endpoint_with_config (cfg) ; let (client , server) = tokio :: join ! (endpoint . connect (endpoint . local_addr () . unwrap () , "localhost") . unwrap () , async { endpoint . accept () . await . unwrap () . await }) ; let client = client . unwrap () ; let server = server . unwrap () ; tokio :: join ! (async { client . open_uni () . await . unwrap () ; } , async { client . open_uni () . await . unwrap () ; } , async { client . open_uni () . await . unwrap () ; } , async { server . accept_uni () . await . unwrap () ; server . accept_uni () . await . unwrap () ; }) ; }
};
}
