// Generated macro for impl_36 (impl)
macro_rules! Depcrate_decode_strimpl_36 {
() => {
// Module: crate::decode::str
// Provides: {"impl_36"}
// Dependencies: {}
impl < E : RmpReadErr > From < ValueReadError < E > > for DecodeStringError < '_ , E > { # [cold] fn from (err : ValueReadError < E >) -> Self { match err { ValueReadError :: InvalidMarkerRead (err) => DecodeStringError :: InvalidMarkerRead (err) , ValueReadError :: InvalidDataRead (err) => DecodeStringError :: InvalidDataRead (err) , ValueReadError :: TypeMismatch (marker) => DecodeStringError :: TypeMismatch (marker) , } } }
};
}
