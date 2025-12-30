// Generated macro for close_endpoint (function)
macro_rules! Depcrate_testsclose_endpoint {
() => {
// Module: crate::tests
// Provides: {"close_endpoint"}
// Dependencies: {}
# [tokio :: test] async fn close_endpoint () { let _guard = subscribe () ; let cert = rcgen :: generate_simple_self_signed (vec ! ["localhost" . into ()]) . unwrap () ; let mut roots = RootCertStore :: empty () ; roots . add (cert . cert . into ()) . unwrap () ; let mut endpoint = Endpoint :: client (SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0)) . unwrap () ; endpoint . set_default_client_config (ClientConfig :: with_root_certificates (Arc :: new (roots)) . unwrap ()) ; let conn = endpoint . connect (SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 1234) , "localhost" ,) . unwrap () ; tokio :: spawn (async move { let _ = conn . await ; }) ; let conn = endpoint . connect (SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 1234) , "localhost" ,) . unwrap () ; endpoint . close (0u32 . into () , & []) ; match conn . await { Err (crate :: ConnectionError :: LocallyClosed) => () , Err (e) => panic ! ("unexpected error: {e}") , Ok (_) => { panic ! ("unexpected success") ; } } }
};
}
