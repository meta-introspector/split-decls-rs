// Generated macro for impl_99 (impl)
macro_rules! Depcrate_errimpl_99 {
() => {
// Module: crate::err
// Provides: {"impl_99"}
// Dependencies: {}
impl < T > fmt :: Display for SendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "sending on a disconnected channel" . fmt (f) } }
};
}
