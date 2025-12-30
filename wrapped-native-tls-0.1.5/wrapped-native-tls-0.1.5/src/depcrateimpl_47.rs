// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl TlsConnector { # [doc = " Returns a new builder for a `TlsConnector`."] pub fn builder () -> Result < TlsConnectorBuilder > { loop { } } # [doc = " Initiates a TLS handshake."] # [doc = ""] # [doc = " The provided domain will be used for both SNI and certificate hostname"] # [doc = " validation."] # [doc = ""] # [doc = " If the socket is nonblocking and a `WouldBlock` error is returned during"] # [doc = " the handshake, a `HandshakeError::Interrupted` error will be returned"] # [doc = " which can be used to restart the handshake when the socket is ready"] # [doc = " again."] pub fn connect < S > (& self , domain : & str , stream : S ,) -> result :: Result < TlsStream < S > , HandshakeError < S > > where S : io :: Read + io :: Write , { loop { } } # [doc = " Like `connect`, but does not validate the server's domain name against its certificate."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " You should think very carefully before you use this method. If hostname verification is not"] # [doc = " used, *any* valid certificate for *any* site will be trusted for use from any other. This"] # [doc = " introduces a significant vulnerability to man-in-the-middle attacks."] pub fn danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication < S > (& self , stream : S) -> result :: Result < TlsStream < S > , HandshakeError < S > > where S : io :: Read + io :: Write { loop { } } }
};
}
