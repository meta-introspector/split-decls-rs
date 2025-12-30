// Generated macro for impl_177 (impl)
macro_rules! Depcrate_astimpl_177 {
() => {
// Module: crate::ast
// Provides: {"impl_177"}
// Dependencies: {}
impl < S > IntoInputValue < S > for f64 where f64 : ToScalarValue < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
