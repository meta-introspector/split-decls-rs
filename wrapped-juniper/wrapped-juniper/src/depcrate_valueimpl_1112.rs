// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_valueimpl_1112 {
() => {
// Module: crate::value
// Provides: {"impl_1112"}
// Dependencies: {}
impl < S : ScalarValue > IntoValue < S > for ArcStr where ArcStr : ToScalarValue < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
