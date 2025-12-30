// Generated macro for impl_70 (impl)
macro_rules! Depcrate_decodeimpl_70 {
() => {
// Module: crate::decode
// Provides: {"impl_70"}
// Dependencies: {}
impl From < NumValueReadError > for Error { # [cold] fn from (err : NumValueReadError) -> Self { match err { NumValueReadError :: TypeMismatch (marker) => Self :: TypeMismatch (marker) , NumValueReadError :: InvalidMarkerRead (err) => Self :: InvalidMarkerRead (err) , NumValueReadError :: InvalidDataRead (err) => Self :: InvalidDataRead (err) , NumValueReadError :: OutOfRange => Self :: OutOfRange , } } }
};
}
