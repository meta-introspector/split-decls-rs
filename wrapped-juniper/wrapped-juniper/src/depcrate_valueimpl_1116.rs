// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_valueimpl_1116 {
() => {
// Module: crate::value
// Provides: {"impl_1116"}
// Dependencies: {}
impl < S > IntoValue < S > for bool where bool : ToScalarValue < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
