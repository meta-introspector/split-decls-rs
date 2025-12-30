// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorsimpl_22 {
() => {
// Module: crate::errors
// Provides: {"impl_22"}
// Dependencies: {}
impl Utf8Error { # [doc = " Get the type of error."] pub const fn kind (& self) -> Utf8ErrorKind { self . kind } # [cfg (not (feature = "std"))] # [allow (missing_docs)] pub const fn description (& self) -> & 'static str { utf8_error_description (self . kind) } }
};
}
