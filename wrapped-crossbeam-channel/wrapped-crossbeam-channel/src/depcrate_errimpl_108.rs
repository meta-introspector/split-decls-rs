// Generated macro for impl_108 (impl)
macro_rules! Depcrate_errimpl_108 {
() => {
// Module: crate::err
// Provides: {"impl_108"}
// Dependencies: {}
impl < T > fmt :: Display for SendTimeoutError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Timeout (..) => "timed out waiting on send operation" . fmt (f) , Self :: Disconnected (..) => "sending on a disconnected channel" . fmt (f) , } } }
};
}
