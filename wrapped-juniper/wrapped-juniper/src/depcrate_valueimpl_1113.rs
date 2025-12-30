// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_valueimpl_1113 {
() => {
// Module: crate::value
// Provides: {"impl_1113"}
// Dependencies: {}
impl < S : ScalarValue > IntoValue < S > for CompactString where CompactString : ToScalarValue < S > , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
