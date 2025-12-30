// Generated macro for impl_174 (impl)
macro_rules! Depcrate_astimpl_174 {
() => {
// Module: crate::ast
// Provides: {"impl_174"}
// Dependencies: {}
impl < S : ScalarValue > IntoInputValue < S > for ArcStr where ArcStr : ToScalarValue < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
