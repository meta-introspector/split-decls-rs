// Generated macro for impl_19 (impl)
macro_rules! Depcrate_forgiving_base64impl_19 {
() => {
// Module: crate::forgiving_base64
// Provides: {"impl_19"}
// Dependencies: {}
impl From < DecodeError < Impossible > > for InvalidBase64 { fn from (e : DecodeError < Impossible >) -> Self { match e { DecodeError :: InvalidBase64 (e) => e , DecodeError :: WriteError (e) => match e { } , } } }
};
}
