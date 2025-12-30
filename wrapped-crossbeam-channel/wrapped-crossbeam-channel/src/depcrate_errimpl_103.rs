// Generated macro for impl_103 (impl)
macro_rules! Depcrate_errimpl_103 {
() => {
// Module: crate::err
// Provides: {"impl_103"}
// Dependencies: {}
impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Full (..) => "sending on a full channel" . fmt (f) , Self :: Disconnected (..) => "sending on a disconnected channel" . fmt (f) , } } }
};
}
