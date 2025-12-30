// Generated macro for impl_464 (impl)
macro_rules! Depcrate_from_metaimpl_464 {
() => {
// Module: crate::from_meta
// Provides: {"impl_464"}
// Dependencies: {}
# [doc = " Parses the meta-item, and in case of error preserves a copy of the input for"] # [doc = " later analysis."] impl < T : FromMeta > FromMeta for :: std :: result :: Result < T , Meta > { fn from_meta (item : & Meta) -> Result < Self > { T :: from_meta (item) . map (Ok) . or_else (| _ | Ok (Err (item . clone ()))) } }
};
}
