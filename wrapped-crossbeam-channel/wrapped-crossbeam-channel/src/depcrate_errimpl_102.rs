// Generated macro for impl_102 (impl)
macro_rules! Depcrate_errimpl_102 {
() => {
// Module: crate::err
// Provides: {"impl_102"}
// Dependencies: {}
impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Full (..) => "Full(..)" . fmt (f) , Self :: Disconnected (..) => "Disconnected(..)" . fmt (f) , } } }
};
}
