// Generated macro for impl_57 (impl)
macro_rules! Depcrate_ordered_mapimpl_57 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_57"}
// Dependencies: {}
impl < K , V > fmt :: Debug for OrderedMap < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_map () . entries (self . entries ()) . finish () } }
};
}
