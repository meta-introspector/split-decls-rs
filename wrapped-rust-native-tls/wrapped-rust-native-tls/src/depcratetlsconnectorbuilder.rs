// Generated macro for TlsConnectorBuilder (struct)
macro_rules! DepcrateTlsConnectorBuilder {
() => {
// Module: crate
// Provides: {"TlsConnectorBuilder"}
// Dependencies: {}
# [doc = " A builder for `TlsConnector`s."] # [doc = ""] # [doc = " You can get one from [`TlsConnector::builder()`](TlsConnector::builder)"] pub struct TlsConnectorBuilder { identity : Option < Identity > , min_protocol : Option < Protocol > , max_protocol : Option < Protocol > , root_certificates : Vec < Certificate > , accept_invalid_certs : bool , accept_invalid_hostnames : bool , use_sni : bool , disable_built_in_roots : bool , # [cfg (feature = "alpn")] alpn : Vec < String > , }
};
}
