// Generated macro for impl_316 (impl)
macro_rules! Depcrate_mapimpl_316 {
() => {
// Module: crate::map
// Provides: {"impl_316"}
// Dependencies: {}
impl < K : Debug , V : Debug > fmt :: Debug for Iter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
