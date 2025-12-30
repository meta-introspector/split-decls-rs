// Generated macro for impl_47 (impl)
macro_rules! Depcrate_baseimpl_47 {
() => {
// Module: crate::base
// Provides: {"impl_47"}
// Dependencies: {}
impl < K , V > fmt :: Debug for RefEntry < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("RefEntry") . field (self . key ()) . field (self . value ()) . finish () } }
};
}
