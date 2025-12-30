// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl TlsConnector { # [doc = " Returns a new connector with default settings."] pub fn new () -> Result < TlsConnector > { TlsConnector :: builder () . build () } # [doc = " Returns a new builder for a `TlsConnector`."] pub fn builder () -> TlsConnectorBuilder { TlsConnectorBuilder { identity : None , min_protocol : Some (Protocol :: Tlsv10) , max_protocol : None , root_certificates : vec ! [] , use_sni : true , accept_invalid_certs : false , accept_invalid_hostnames : false , disable_built_in_roots : false , # [cfg (feature = "alpn")] alpn : vec ! [] , } } # [doc = " Initiates a TLS handshake."] # [doc = ""] # [doc = " The provided domain will be used for both SNI and certificate hostname"] # [doc = " validation."] # [doc = ""] # [doc = " If the socket is nonblocking and a `WouldBlock` error is returned during"] # [doc = " the handshake, a `HandshakeError::WouldBlock` error will be returned"] # [doc = " which can be used to restart the handshake when the socket is ready"] # [doc = " again."] # [doc = ""] # [doc = " The domain is ignored if both SNI and hostname verification are"] # [doc = " disabled."] pub fn connect < S > (& self , domain : & str , stream : S ,) -> result :: Result < TlsStream < S > , HandshakeError < S > > where S : io :: Read + io :: Write , { let s = self . 0 . connect (domain , stream) ? ; Ok (TlsStream (s)) } }
};
}
