// Generated macro for handshake_timeout (function)
macro_rules! Depcrate_testshandshake_timeout {
() => {
// Module: crate::tests
// Provides: {"handshake_timeout"}
// Dependencies: {}
# [test] fn handshake_timeout () { let _guard = subscribe () ; let runtime = rt_threaded () ; let client = { let _guard = runtime . enter () ; Endpoint :: client (SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0)) . unwrap () } ; let cert = rcgen :: generate_simple_self_signed (vec ! ["localhost" . into ()]) . unwrap () ; let mut roots = RootCertStore :: empty () ; roots . add (cert . cert . into ()) . unwrap () ; let mut client_config = crate :: ClientConfig :: with_root_certificates (Arc :: new (roots)) . unwrap () ; const IDLE_TIMEOUT : Duration = Duration :: from_millis (500) ; let mut transport_config = crate :: TransportConfig :: default () ; transport_config . max_idle_timeout (Some (IDLE_TIMEOUT . try_into () . unwrap ())) . initial_rtt (Duration :: from_millis (10)) ; client_config . transport_config (Arc :: new (transport_config)) ; let start = Instant :: now () ; runtime . block_on (async move { match client . connect_with (client_config , SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 1) , "localhost" ,) . unwrap () . await { Err (crate :: ConnectionError :: TimedOut) => { } Err (e) => panic ! ("unexpected error: {e:?}") , Ok (_) => panic ! ("unexpected success") , } }) ; let dt = start . elapsed () ; assert ! (dt > IDLE_TIMEOUT && dt < 2 * IDLE_TIMEOUT) ; }
};
}
