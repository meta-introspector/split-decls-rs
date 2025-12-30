// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorsimpl_14 {
() => {
// Module: crate::errors
// Provides: {"impl_14"}
// Dependencies: {}
impl InvalidValue { # [doc = " Create an [`Error::ParamValueInvalid`] which warps this error."] pub fn param_error (self) -> Error { Error :: ParamValueInvalid (self) } # [doc = " Create an [`Error::SaltInvalid`] which wraps this error."] pub fn salt_error (self) -> Error { Error :: SaltInvalid (self) } }
};
}
