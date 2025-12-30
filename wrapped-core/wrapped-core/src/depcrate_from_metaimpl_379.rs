// Generated macro for impl_379 (impl)
macro_rules! Depcrate_from_metaimpl_379 {
() => {
// Module: crate::from_meta
// Provides: {"impl_379"}
// Dependencies: {}
impl FromMeta for AtomicBool { fn from_meta (mi : & Meta) -> Result < Self > { FromMeta :: from_meta (mi) . map (AtomicBool :: new) . map_err (| e | e . with_span (mi)) } }
};
}
