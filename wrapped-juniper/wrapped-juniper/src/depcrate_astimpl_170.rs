// Generated macro for impl_170 (impl)
macro_rules! Depcrate_astimpl_170 {
() => {
// Module: crate::ast
// Provides: {"impl_170"}
// Dependencies: {}
impl < T , S > IntoInputValue < S > for Option < T > where T : IntoInputValue < S > , { fn into_input_value (self) -> InputValue < S > { match self { Some (v) => v . into_input_value () , None => InputValue :: Null , } } }
};
}
