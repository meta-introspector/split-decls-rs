// Generated macro for impl_178 (impl)
macro_rules! Depcrate_astimpl_178 {
() => {
// Module: crate::ast
// Provides: {"impl_178"}
// Dependencies: {}
impl < S > IntoInputValue < S > for bool where bool : ToScalarValue < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
