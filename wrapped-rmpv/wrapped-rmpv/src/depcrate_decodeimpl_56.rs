// Generated macro for impl_56 (impl)
macro_rules! Depcrate_decodeimpl_56 {
() => {
// Module: crate::decode
// Provides: {"impl_56"}
// Dependencies: {}
impl From < ValueReadError > for Error { # [cold] fn from (err : ValueReadError) -> Self { match err { ValueReadError :: InvalidMarkerRead (err) => Self :: InvalidMarkerRead (err) , ValueReadError :: InvalidDataRead (err) => Self :: InvalidDataRead (err) , ValueReadError :: TypeMismatch (..) => { Self :: InvalidMarkerRead (io :: Error :: new (ErrorKind :: Other , "type mismatch")) } } } }
};
}
