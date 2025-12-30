// Generated macro for impl_175 (impl)
macro_rules! Depcrate_astimpl_175 {
() => {
// Module: crate::ast
// Provides: {"impl_175"}
// Dependencies: {}
impl < S : ScalarValue > IntoInputValue < S > for CompactString where CompactString : ToScalarValue < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
