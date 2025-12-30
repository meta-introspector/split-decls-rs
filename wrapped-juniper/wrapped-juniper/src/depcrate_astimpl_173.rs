// Generated macro for impl_173 (impl)
macro_rules! Depcrate_astimpl_173 {
() => {
// Module: crate::ast
// Provides: {"impl_173"}
// Dependencies: {}
impl < S > IntoInputValue < S > for Cow < '_ , str > where for < 'a > & 'a str : IntoInputValue < S > , String : IntoInputValue < S > , { fn into_input_value (self) -> InputValue < S > { match self { Cow :: Borrowed (s) => s . into_input_value () , Cow :: Owned (s) => s . into_input_value () , } } }
};
}
