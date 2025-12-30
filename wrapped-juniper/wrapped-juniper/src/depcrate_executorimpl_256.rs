// Generated macro for impl_256 (impl)
macro_rules! Depcrate_executorimpl_256 {
() => {
// Module: crate::executor
// Provides: {"impl_256"}
// Dependencies: {}
impl < S1 : ScalarValue , S2 : ScalarValue > IntoFieldError < S2 > for FieldError < S1 > { fn into_field_error (self) -> FieldError < S2 > { self . map_scalar_value () } }
};
}
