// Generated macro for connect_client (function)
macro_rules! Depcrateconnect_client {
() => {
// Module: crate
// Provides: {"connect_client"}
// Dependencies: {}
# [doc = " Create a client endpoint and client connection"] pub async fn connect_client (server_addr : SocketAddr , server_cert : CertificateDer < '_ > , opt : Opt ,) -> Result < (quinn :: Endpoint , quinn :: Connection) > { let endpoint = quinn :: Endpoint :: client (SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: LOCALHOST) , 0)) . unwrap () ; let mut roots = RootCertStore :: empty () ; roots . add (server_cert) ? ; let default_provider = rustls :: crypto :: ring :: default_provider () ; let provider = rustls :: crypto :: CryptoProvider { cipher_suites : vec ! [opt . cipher . as_rustls ()] , .. default_provider } ; let crypto = rustls :: ClientConfig :: builder_with_provider (provider . into ()) . with_protocol_versions (& [& rustls :: version :: TLS13]) . unwrap () . with_root_certificates (roots) . with_no_client_auth () ; let mut client_config = quinn :: ClientConfig :: new (Arc :: new (QuicClientConfig :: try_from (crypto) ?)) ; client_config . transport_config (Arc :: new (transport_config (& opt))) ; let connection = endpoint . connect_with (client_config , server_addr , "localhost") . unwrap () . await . context ("unable to connect") ? ; trace ! ("connected") ; Ok ((endpoint , connection)) }
};
}
