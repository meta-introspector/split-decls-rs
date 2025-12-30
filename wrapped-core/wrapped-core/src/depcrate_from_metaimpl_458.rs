// Generated macro for impl_458 (impl)
macro_rules! Depcrate_from_metaimpl_458 {
() => {
// Module: crate::from_meta
// Provides: {"impl_458"}
// Dependencies: {}
impl < T : FromMeta > FromMeta for Result < T > { fn from_none () -> Option < Self > { T :: from_none () . map (Ok) } fn from_list (items : & [NestedMeta]) -> Result < Self > { Ok (FromMeta :: from_list (items)) } fn from_meta (item : & Meta) -> Result < Self > { Ok (FromMeta :: from_meta (item)) } }
};
}
