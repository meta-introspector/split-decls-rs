// Generated macro for impl_33 (impl)
macro_rules! Depcrate_errorimpl_33 {
() => {
// Module: crate::error
// Provides: {"impl_33"}
// Dependencies: {}
impl ParameterError { # [doc = " Construct a `ParameterError` directly from a corresponding kind."] # [must_use] pub fn from_kind (kind : ParameterErrorKind) -> Self { ParameterError { kind , underlying : None , } } # [doc = " Returns the corresponding `ParameterErrorKind` of the error."] # [must_use] pub fn kind (& self) -> ParameterErrorKind { self . kind . clone () } }
};
}
