// Generated macro for impl_172 (impl)
macro_rules! Depcrate_astimpl_172 {
() => {
// Module: crate::ast
// Provides: {"impl_172"}
// Dependencies: {}
impl < S > IntoInputValue < S > for String where String : Into < S > , { fn into_input_value (self) -> InputValue < S > { InputValue :: Scalar (self . into ()) } }
};
}
