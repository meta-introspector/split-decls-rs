// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_value_scalarimpl_1087 {
() => {
// Module: crate::value::scalar
// Provides: {"impl_1087"}
// Dependencies: {}
impl < 's , S : ScalarValue > FromScalarValue < 's , S > for & 's Scalar < S > { type Error = Infallible ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { Ok (v . into ()) } }
};
}
