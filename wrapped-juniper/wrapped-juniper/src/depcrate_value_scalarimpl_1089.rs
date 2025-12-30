// Generated macro for impl_1089 (impl)
macro_rules! Depcrate_value_scalarimpl_1089 {
() => {
// Module: crate::value::scalar
// Provides: {"impl_1089"}
// Dependencies: {}
impl < 'a , S : ScalarValue > IntoFieldError < S > for WrongInputScalarTypeError < 'a , S > { fn into_field_error (self) -> FieldError < S > { FieldError :: < S > :: from (self) } }
};
}
