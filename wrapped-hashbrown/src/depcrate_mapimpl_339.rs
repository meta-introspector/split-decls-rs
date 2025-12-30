// Generated macro for impl_339 (impl)
macro_rules! Depcrate_mapimpl_339 {
() => {
// Module: crate::map
// Provides: {"impl_339"}
// Dependencies: {}
impl < K , V : Debug > fmt :: Debug for Values < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
