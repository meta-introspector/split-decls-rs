// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorsimpl_12 {
() => {
// Module: crate::errors
// Provides: {"impl_12"}
// Dependencies: {}
impl From < base64ct :: InvalidLengthError > for Error { fn from (_ : base64ct :: InvalidLengthError) -> Error { Error :: B64Encoding (B64Error :: InvalidLength) } }
};
}
