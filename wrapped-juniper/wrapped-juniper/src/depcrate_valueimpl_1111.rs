// Generated macro for impl_1111 (impl)
macro_rules! Depcrate_valueimpl_1111 {
() => {
// Module: crate::value
// Provides: {"impl_1111"}
// Dependencies: {}
impl < S > IntoValue < S > for Cow < '_ , str > where for < 'a > & 'a str : IntoValue < S > , String : IntoValue < S > , { fn into_value (self) -> Value < S > { match self { Cow :: Borrowed (s) => s . into_value () , Cow :: Owned (s) => s . into_value () , } } }
};
}
