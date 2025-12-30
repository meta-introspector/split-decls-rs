// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < S > MidHandshakeTlsStream < S > where S : io :: Read + io :: Write , { # [doc = " Restarts the handshake process."] # [doc = ""] # [doc = " If the handshake completes successfully then the negotiated stream is"] # [doc = " returned. If there is a problem, however, then an error is returned."] # [doc = " Note that the error may not be fatal. For example if the underlying"] # [doc = " stream is an asynchronous one then `HandshakeError::WouldBlock` may"] # [doc = " just mean to wait for more I/O to happen later."] pub fn handshake (self) -> result :: Result < TlsStream < S > , HandshakeError < S > > { match self . 0 . handshake () { Ok (s) => Ok (TlsStream (s)) , Err (e) => Err (e . into ()) , } } }
};
}
