// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl TlsAcceptor { # [doc = " Creates a acceptor with default settings."] # [doc = ""] # [doc = " The identity acts as the server's private key/certificate chain."] pub fn new (identity : Identity) -> Result < TlsAcceptor > { TlsAcceptor :: builder (identity) . build () } # [doc = " Returns a new builder for a `TlsAcceptor`."] # [doc = ""] # [doc = " The identity acts as the server's private key/certificate chain."] pub fn builder (identity : Identity) -> TlsAcceptorBuilder { TlsAcceptorBuilder { identity , min_protocol : Some (Protocol :: Tlsv10) , max_protocol : None , } } # [doc = " Initiates a TLS handshake."] # [doc = ""] # [doc = " If the socket is nonblocking and a `WouldBlock` error is returned during"] # [doc = " the handshake, a `HandshakeError::WouldBlock` error will be returned"] # [doc = " which can be used to restart the handshake when the socket is ready"] # [doc = " again."] pub fn accept < S > (& self , stream : S) -> result :: Result < TlsStream < S > , HandshakeError < S > > where S : io :: Read + io :: Write , { match self . 0 . accept (stream) { Ok (s) => Ok (TlsStream (s)) , Err (e) => Err (e . into ()) , } } }
};
}
