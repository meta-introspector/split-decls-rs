// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorsimpl_27 {
() => {
// Module: crate::errors
// Provides: {"impl_27"}
// Dependencies: {}
impl StdError for SourceDestError { fn cause (& self) -> Option < & dyn StdError > { self . source () } # [cfg (not (feature = "expose_original_error"))] fn source (& self) -> Option < & (dyn StdError + 'static) > { None } # [cfg (feature = "expose_original_error")] fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . source) } }
};
}
