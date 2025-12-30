// Generated macro for impl_336 (impl)
macro_rules! Depcrate_mapimpl_336 {
() => {
// Module: crate::map
// Provides: {"impl_336"}
// Dependencies: {}
impl < K : Debug , V > fmt :: Debug for Keys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
