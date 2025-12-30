// Generated macro for impl_372 (impl)
macro_rules! Depcrate_mapimpl_372 {
() => {
// Module: crate::map
// Provides: {"impl_372"}
// Dependencies: {}
impl < K , V > fmt :: Debug for IterMut < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
