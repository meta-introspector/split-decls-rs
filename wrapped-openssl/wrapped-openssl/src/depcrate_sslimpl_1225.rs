// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_sslimpl_1225 {
() => {
// Module: crate::ssl
// Provides: {"impl_1225"}
// Dependencies: {}
impl < S > MidHandshakeSslStream < S > where S : Read + Write , { # [doc = " Restarts the handshake process."] # [doc = ""] # [corresponds (SSL_do_handshake)] pub fn handshake (mut self) -> Result < SslStream < S > , HandshakeError < S > > { match self . stream . do_handshake () { Ok (()) => Ok (self . stream) , Err (error) => { self . error = error ; match self . error . code () { ErrorCode :: WANT_READ | ErrorCode :: WANT_WRITE => { Err (HandshakeError :: WouldBlock (self)) } _ => Err (HandshakeError :: Failure (self)) , } } } } }
};
}
