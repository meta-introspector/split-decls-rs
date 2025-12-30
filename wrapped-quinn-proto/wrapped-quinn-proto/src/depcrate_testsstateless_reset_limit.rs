// Generated macro for stateless_reset_limit (function)
macro_rules! Depcrate_testsstateless_reset_limit {
() => {
// Module: crate::tests
// Provides: {"stateless_reset_limit"}
// Dependencies: {}
# [doc = " Verify that stateless resets are rate-limited"] # [test] fn stateless_reset_limit () { let _guard = subscribe () ; let remote = SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 42) ; let mut endpoint_config = EndpointConfig :: default () ; endpoint_config . cid_generator (move | | Box :: new (RandomConnectionIdGenerator :: new (8))) ; let endpoint_config = Arc :: new (endpoint_config) ; let mut endpoint = Endpoint :: new (endpoint_config . clone () , Some (Arc :: new (server_config ())) , true , None ,) ; let time = Instant :: now () ; let mut buf = Vec :: new () ; let event = endpoint . handle (time , remote , None , None , [0u8 ; 1024] [..] . into () , & mut buf) ; assert ! (matches ! (event , Some (DatagramEvent :: Response (_)))) ; let event = endpoint . handle (time , remote , None , None , [0u8 ; 1024] [..] . into () , & mut buf) ; assert ! (event . is_none ()) ; let event = endpoint . handle (time + endpoint_config . min_reset_interval - Duration :: from_nanos (1) , remote , None , None , [0u8 ; 1024] [..] . into () , & mut buf ,) ; assert ! (event . is_none ()) ; let event = endpoint . handle (time + endpoint_config . min_reset_interval , remote , None , None , [0u8 ; 1024] [..] . into () , & mut buf ,) ; assert ! (matches ! (event , Some (DatagramEvent :: Response (_)))) ; }
};
}
