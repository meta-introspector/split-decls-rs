// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < S > error :: Error for HandshakeError < S > where S : Any + fmt :: Debug , { fn description (& self) -> & str { match * self { HandshakeError :: Failure (ref e) => e . description () , HandshakeError :: Interrupted (_) => "the handshake process was interrupted" , } } fn cause (& self) -> Option < & error :: Error > { match * self { HandshakeError :: Failure (ref e) => Some (e) , HandshakeError :: Interrupted (_) => None , } } }
};
}
