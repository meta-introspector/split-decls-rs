// Generated macro for server_endpoint (function)
macro_rules! Depcrateserver_endpoint {
() => {
// Module: crate
// Provides: {"server_endpoint"}
// Dependencies: {}
# [doc = " Creates a server endpoint which runs on the given runtime"] pub fn server_endpoint (rt : & tokio :: runtime :: Runtime , cert : CertificateDer < 'static > , key : PrivateKeyDer < 'static > , opt : & Opt ,) -> (SocketAddr , quinn :: Endpoint) { let cert_chain = vec ! [cert] ; let mut server_config = quinn :: ServerConfig :: with_single_cert (cert_chain , key) . unwrap () ; server_config . transport = Arc :: new (transport_config (opt)) ; let endpoint = { let _guard = rt . enter () ; quinn :: Endpoint :: server (server_config , SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: LOCALHOST) , 0) ,) . unwrap () } ; let server_addr = endpoint . local_addr () . unwrap () ; (server_addr , endpoint) }
};
}
