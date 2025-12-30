// Generated macro for impl_457 (impl)
macro_rules! Depcrate_from_metaimpl_457 {
() => {
// Module: crate::from_meta
// Provides: {"impl_457"}
// Dependencies: {}
impl < T : FromMeta > FromMeta for Option < T > { fn from_none () -> Option < Self > { Some (None) } fn from_meta (item : & Meta) -> Result < Self > { FromMeta :: from_meta (item) . map (Some) } }
};
}
