// Generated macro for impl_38 (impl)
macro_rules! Depcrate_errorimpl_38 {
() => {
// Module: crate::error
// Provides: {"impl_38"}
// Dependencies: {}
impl From < ImageFormatHint > for UnsupportedError { fn from (hint : ImageFormatHint) -> Self { UnsupportedError { format : hint . clone () , kind : UnsupportedErrorKind :: Format (hint) , } } }
};
}
