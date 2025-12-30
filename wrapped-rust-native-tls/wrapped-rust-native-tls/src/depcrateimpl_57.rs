// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < S > From < imp :: HandshakeError < S > > for HandshakeError < S > { fn from (e : imp :: HandshakeError < S >) -> HandshakeError < S > { match e { imp :: HandshakeError :: Failure (e) => HandshakeError :: Failure (Error (e)) , imp :: HandshakeError :: WouldBlock (s) => { HandshakeError :: WouldBlock (MidHandshakeTlsStream (s)) } } } }
};
}
