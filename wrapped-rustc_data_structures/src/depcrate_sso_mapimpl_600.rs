// Generated macro for impl_600 (impl)
macro_rules! Depcrate_sso_mapimpl_600 {
() => {
// Module: crate::sso::map
// Provides: {"impl_600"}
// Dependencies: {}
impl < K , V > fmt :: Debug for SsoHashMap < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
