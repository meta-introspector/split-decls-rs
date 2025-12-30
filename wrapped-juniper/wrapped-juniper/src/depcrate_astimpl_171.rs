// Generated macro for impl_171 (impl)
macro_rules! Depcrate_astimpl_171 {
() => {
// Module: crate::ast
// Provides: {"impl_171"}
// Dependencies: {}
impl < T , S > IntoInputValue < S > for & T where T : ToScalarValue < S > + ? Sized , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
