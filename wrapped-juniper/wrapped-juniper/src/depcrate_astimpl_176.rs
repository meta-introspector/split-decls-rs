// Generated macro for impl_176 (impl)
macro_rules! Depcrate_astimpl_176 {
() => {
// Module: crate::ast
// Provides: {"impl_176"}
// Dependencies: {}
impl < S > IntoInputValue < S > for i32 where i32 : ToScalarValue < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
