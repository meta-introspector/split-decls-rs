// Generated macro for impl_66 (impl)
macro_rules! Depcrate_header_mapimpl_66 {
() => {
// Module: crate::header::map
// Provides: {"impl_66"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for HeaderMap < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
