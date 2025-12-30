// Generated macro for impl_993 (impl)
macro_rules! Depcrate_ssl_connectorimpl_993 {
() => {
// Module: crate::ssl::connector
// Provides: {"impl_993"}
// Dependencies: {}
impl SslConnector { # [doc = " Creates a new builder for TLS connections."] # [doc = ""] # [doc = " The default configuration is subject to change, and is currently derived from Python."] pub fn builder (method : SslMethod) -> Result < SslConnectorBuilder , ErrorStack > { let mut ctx = ctx (method) ? ; ctx . set_default_verify_paths () ? ; ctx . set_cipher_list ("DEFAULT:!aNULL:!eNULL:!MD5:!3DES:!DES:!RC4:!IDEA:!SEED:!aDSS:!SRP:!PSK" ,) ? ; setup_verify (& mut ctx) ; Ok (SslConnectorBuilder (ctx)) } # [doc = " Initiates a client-side TLS session on a stream."] # [doc = ""] # [doc = " The domain is used for SNI and hostname verification."] pub fn connect < S > (& self , domain : & str , stream : S) -> Result < SslStream < S > , HandshakeError < S > > where S : Read + Write , { self . configure () ? . connect (domain , stream) } # [doc = " Returns a structure allowing for configuration of a single TLS session before connection."] pub fn configure (& self) -> Result < ConnectConfiguration , ErrorStack > { Ssl :: new (& self . 0) . map (| ssl | ConnectConfiguration { ssl , sni : true , verify_hostname : true , }) } # [doc = " Consumes the `SslConnector`, returning the inner raw `SslContext`."] pub fn into_context (self) -> SslContext { self . 0 } # [doc = " Returns a shared reference to the inner raw `SslContext`."] pub fn context (& self) -> & SslContextRef { & self . 0 } }
};
}
