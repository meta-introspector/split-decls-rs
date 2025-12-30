// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_ssl_errorimpl_1029 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1029"}
// Dependencies: {}
impl < S : fmt :: Debug > StdError for HandshakeError < S > { fn source (& self) -> Option < & (dyn StdError + 'static) > { match * self { HandshakeError :: SetupFailure (ref e) => Some (e) , HandshakeError :: Failure (ref s) | HandshakeError :: WouldBlock (ref s) => Some (s . error ()) , } } }
};
}
