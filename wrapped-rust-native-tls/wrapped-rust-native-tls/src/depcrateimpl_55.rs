// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl < S > error :: Error for HandshakeError < S > where S : Any + fmt :: Debug , { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { HandshakeError :: Failure (ref e) => Some (e) , HandshakeError :: WouldBlock (_) => None , } } }
};
}
