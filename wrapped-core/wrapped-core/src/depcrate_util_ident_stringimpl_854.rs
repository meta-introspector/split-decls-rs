// Generated macro for impl_854 (impl)
macro_rules! Depcrate_util_ident_stringimpl_854 {
() => {
// Module: crate::util::ident_string
// Provides: {"impl_854"}
// Dependencies: {}
impl FromMeta for IdentString { fn from_meta (item : & Meta) -> Result < Self > { Ident :: from_meta (item) . map (IdentString :: from) } }
};
}
