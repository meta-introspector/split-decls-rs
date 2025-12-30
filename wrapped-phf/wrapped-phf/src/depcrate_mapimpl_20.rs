// Generated macro for impl_20 (impl)
macro_rules! Depcrate_mapimpl_20 {
() => {
// Module: crate::map
// Provides: {"impl_20"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Map < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_map () . entries (self . entries ()) . finish () } }
};
}
