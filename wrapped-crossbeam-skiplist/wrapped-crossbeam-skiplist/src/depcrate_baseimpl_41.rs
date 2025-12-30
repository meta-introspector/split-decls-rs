// Generated macro for impl_41 (impl)
macro_rules! Depcrate_baseimpl_41 {
() => {
// Module: crate::base
// Provides: {"impl_41"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Entry < '_ , '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Entry") . field (self . key ()) . field (self . value ()) . finish () } }
};
}
