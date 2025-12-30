// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl StdError for StringTypedError { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . source . as_ref () . map (| err | err as & dyn StdError) } }
};
}
