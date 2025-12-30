// Generated macro for impl_171 (impl)
macro_rules! Depcrate_errorimpl_171 {
() => {
// Module: crate::error
// Provides: {"impl_171"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . inner . cause . as_ref () . map (| cause | & * * cause as & (dyn StdError + 'static)) } }
};
}
