// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl TlsAcceptor { # [doc = " Returns a new builder for a `TlsAcceptor`."] # [doc = ""] # [doc = " This builder is created with a key/certificate pair in the `pkcs12`"] # [doc = " archived passed in. The returned builder will use that key/certificate"] # [doc = " to send to clients which it connects to."] pub fn builder (pkcs12 : Pkcs12) -> Result < TlsAcceptorBuilder > { loop { } } # [doc = " Initiates a TLS handshake."] # [doc = ""] # [doc = " If the socket is nonblocking and a `WouldBlock` error is returned during"] # [doc = " the handshake, a `HandshakeError::Interrupted` error will be returned"] # [doc = " which can be used to restart the handshake when the socket is ready"] # [doc = " again."] pub fn accept < S > (& self , stream : S) -> result :: Result < TlsStream < S > , HandshakeError < S > > where S : io :: Read + io :: Write , { loop { } } }
};
}
