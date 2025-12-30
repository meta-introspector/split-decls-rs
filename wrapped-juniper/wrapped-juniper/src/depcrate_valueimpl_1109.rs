// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_valueimpl_1109 {
() => {
// Module: crate::value
// Provides: {"impl_1109"}
// Dependencies: {}
impl < T , S > IntoValue < S > for & T where T : ToScalarValue < S > + ? Sized , { fn into_value (self) -> Value < S > { Value :: Scalar (self . to_scalar_value ()) } }
};
}
