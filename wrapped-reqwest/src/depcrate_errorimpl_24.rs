// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . inner . source . as_ref () . map (| e | & * * e as _) } }
};
}
