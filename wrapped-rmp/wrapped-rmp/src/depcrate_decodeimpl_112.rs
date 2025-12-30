// Generated macro for impl_112 (impl)
macro_rules! Depcrate_decodeimpl_112 {
() => {
// Module: crate::decode
// Provides: {"impl_112"}
// Dependencies: {}
impl < E : RmpReadErr > From < ValueReadError < E > > for NumValueReadError < E > { # [cold] fn from (err : ValueReadError < E >) -> Self { match err { ValueReadError :: InvalidMarkerRead (err) => Self :: InvalidMarkerRead (err) , ValueReadError :: InvalidDataRead (err) => Self :: InvalidDataRead (err) , ValueReadError :: TypeMismatch (err) => Self :: TypeMismatch (err) , } } }
};
}
