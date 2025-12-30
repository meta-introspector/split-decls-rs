// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_valueimpl_1108 {
() => {
// Module: crate::value
// Provides: {"impl_1108"}
// Dependencies: {}
impl < T , S > IntoValue < S > for Option < T > where T : IntoValue < S > , { fn into_value (self) -> Value < S > { match self { Some (v) => v . into_value () , None => Value :: Null , } } }
};
}
