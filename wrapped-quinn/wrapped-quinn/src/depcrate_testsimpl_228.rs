// Generated macro for impl_228 (impl)
macro_rules! Depcrate_testsimpl_228 {
() => {
// Module: crate::tests
// Provides: {"impl_228"}
// Dependencies: {}
impl EndpointFactory { fn new () -> Self { Self { cert : rcgen :: generate_simple_self_signed (vec ! ["localhost" . into ()]) . unwrap () , endpoint_config : EndpointConfig :: default () , } } fn endpoint (& self) -> Endpoint { self . endpoint_with_config (TransportConfig :: default ()) } fn endpoint_with_config (& self , transport_config : TransportConfig) -> Endpoint { let key = PrivateKeyDer :: Pkcs8 (self . cert . signing_key . serialize_der () . into ()) ; let transport_config = Arc :: new (transport_config) ; let mut server_config = crate :: ServerConfig :: with_single_cert (vec ! [self . cert . cert . der () . clone ()] , key) . unwrap () ; server_config . transport_config (transport_config . clone ()) ; let mut roots = rustls :: RootCertStore :: empty () ; roots . add (self . cert . cert . der () . clone ()) . unwrap () ; let mut endpoint = Endpoint :: new (self . endpoint_config . clone () , Some (server_config) , UdpSocket :: bind (SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0)) . unwrap () , Arc :: new (TokioRuntime) ,) . unwrap () ; let mut client_config = ClientConfig :: with_root_certificates (Arc :: new (roots)) . unwrap () ; client_config . transport_config (transport_config) ; endpoint . set_default_client_config (client_config) ; endpoint } }
};
}
