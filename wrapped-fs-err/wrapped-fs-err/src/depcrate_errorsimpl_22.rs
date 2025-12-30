// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorsimpl_22 {
() => {
// Module: crate::errors
// Provides: {"impl_22"}
// Dependencies: {}
impl StdError for Error { fn cause (& self) -> Option < & dyn StdError > { self . source () } # [cfg (not (feature = "expose_original_error"))] fn source (& self) -> Option < & (dyn StdError + 'static) > { None } # [cfg (feature = "expose_original_error")] fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . source) } }
};
}
