// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < S > MidHandshakeTlsStream < S > where S : io :: Read + io :: Write , { # [doc = " Returns a shared reference to the inner stream."] pub fn get_ref (& self) -> & S { loop { } } # [doc = " Returns a mutable reference to the inner stream."] pub fn get_mut (& mut self) -> & mut S { loop { } } # [doc = " Restarts the handshake process."] # [doc = ""] # [doc = " If the handshake completes successfully then the negotiated stream is"] # [doc = " returned. If there is a problem, however, then an error is returned."] # [doc = " Note that the error may not be fatal. For example if the underlying"] # [doc = " stream is an asynchronous one then `HandshakeError::Interrupted` may"] # [doc = " just mean to wait for more I/O to happen later."] pub fn handshake (self) -> result :: Result < TlsStream < S > , HandshakeError < S > > { loop { } } }
};
}
