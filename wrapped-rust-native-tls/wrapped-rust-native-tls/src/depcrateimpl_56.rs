// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < S > fmt :: Display for HandshakeError < S > where S : Any + fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { HandshakeError :: Failure (ref e) => fmt :: Display :: fmt (e , fmt) , HandshakeError :: WouldBlock (_) => fmt . write_str ("the handshake process was interrupted") , } } }
};
}
