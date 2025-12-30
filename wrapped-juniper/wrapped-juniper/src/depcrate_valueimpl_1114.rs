// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_valueimpl_1114 {
() => {
// Module: crate::value
// Provides: {"impl_1114"}
// Dependencies: {}
impl < S > IntoValue < S > for i32 where i32 : ToScalarValue < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
