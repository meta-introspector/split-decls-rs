// Generated macro for impl_17 (impl)
macro_rules! Depcrate_forgiving_base64impl_17 {
() => {
// Module: crate::forgiving_base64
// Provides: {"impl_17"}
// Dependencies: {}
impl < E > From < InvalidBase64Details > for DecodeError < E > { fn from (e : InvalidBase64Details) -> Self { Self :: InvalidBase64 (InvalidBase64 (e)) } }
};
}
