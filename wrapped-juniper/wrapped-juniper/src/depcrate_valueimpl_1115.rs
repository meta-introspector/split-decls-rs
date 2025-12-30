// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_valueimpl_1115 {
() => {
// Module: crate::value
// Provides: {"impl_1115"}
// Dependencies: {}
impl < S > IntoValue < S > for f64 where f64 : ToScalarValue < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
