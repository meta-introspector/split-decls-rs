// Generated macro for impl_16 (impl)
macro_rules! Depcrate_responseimpl_16 {
() => {
// Module: crate::response
// Provides: {"impl_16"}
// Dependencies: {}
impl < S : ScalarValue > IntoResponse for JuniperResponse < S > { fn into_response (self) -> Response { if self . 0 . is_ok () { Json (self . 0) . into_response () } else { (StatusCode :: BAD_REQUEST , Json (self . 0)) . into_response () } } }
};
}
