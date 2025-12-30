// Generated macro for impl_96 (impl)
macro_rules! Depcrate_mapimpl_96 {
() => {
// Module: crate::map
// Provides: {"impl_96"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Entry < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Entry") . field (self . key ()) . field (self . value ()) . finish () } }
};
}
