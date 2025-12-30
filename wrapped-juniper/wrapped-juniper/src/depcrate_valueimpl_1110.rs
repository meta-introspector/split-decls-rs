// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_valueimpl_1110 {
() => {
// Module: crate::value
// Provides: {"impl_1110"}
// Dependencies: {}
impl < S > IntoValue < S > for String where String : Into < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . into ()) } }
};
}
