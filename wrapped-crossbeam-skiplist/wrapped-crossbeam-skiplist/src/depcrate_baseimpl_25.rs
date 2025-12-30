// Generated macro for impl_25 (impl)
macro_rules! Depcrate_baseimpl_25 {
() => {
// Module: crate::base
// Provides: {"impl_25"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Node < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Node") . field (& self . key) . field (& self . value) . finish () } }
};
}
