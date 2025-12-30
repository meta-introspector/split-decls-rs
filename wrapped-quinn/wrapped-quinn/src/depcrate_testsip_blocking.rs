// Generated macro for ip_blocking (function)
macro_rules! Depcrate_testsip_blocking {
() => {
// Module: crate::tests
// Provides: {"ip_blocking"}
// Dependencies: {}
# [tokio :: test] async fn ip_blocking () { let _guard = subscribe () ; let endpoint_factory = EndpointFactory :: new () ; let client_1 = endpoint_factory . endpoint () ; let client_1_addr = client_1 . local_addr () . unwrap () ; let client_2 = endpoint_factory . endpoint () ; let server = endpoint_factory . endpoint () ; let server_addr = server . local_addr () . unwrap () ; let server_task = tokio :: spawn (async move { loop { let accepting = server . accept () . await . unwrap () ; if accepting . remote_address () == client_1_addr { accepting . refuse () ; } else if accepting . remote_address_validated () { accepting . await . expect ("connection") ; } else { accepting . retry () . unwrap () ; } } }) ; tokio :: join ! (async move { let e = client_1 . connect (server_addr , "localhost") . unwrap () . await . expect_err ("server should have blocked this") ; assert ! (matches ! (e , crate :: ConnectionError :: ConnectionClosed (_)) , "wrong error") ; } , async move { client_2 . connect (server_addr , "localhost") . unwrap () . await . expect ("connect") ; }) ; server_task . abort () ; }
};
}
