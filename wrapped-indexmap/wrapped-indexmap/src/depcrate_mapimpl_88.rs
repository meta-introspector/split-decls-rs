// Generated macro for impl_88 (impl)
macro_rules! Depcrate_mapimpl_88 {
() => {
// Module: crate::map
// Provides: {"impl_88"}
// Dependencies: {}
impl < K , V , S > fmt :: Debug for IndexMap < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { # [cfg (not (feature = "test_debug"))] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } # [cfg (feature = "test_debug")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IndexMap") . field ("core" , & self . core) . finish () } }
};
}
