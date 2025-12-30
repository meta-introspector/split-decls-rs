// Generated macro for impl_69 (impl)
macro_rules! Depcrate_decodeimpl_69 {
() => {
// Module: crate::decode
// Provides: {"impl_69"}
// Dependencies: {}
impl From < ValueReadError > for Error { # [cold] fn from (err : ValueReadError) -> Self { match err { ValueReadError :: TypeMismatch (marker) => Self :: TypeMismatch (marker) , ValueReadError :: InvalidMarkerRead (err) => Self :: InvalidMarkerRead (err) , ValueReadError :: InvalidDataRead (err) => Self :: InvalidDataRead (err) , } } }
};
}
