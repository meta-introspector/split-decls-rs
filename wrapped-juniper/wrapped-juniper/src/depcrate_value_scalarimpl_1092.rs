// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_value_scalarimpl_1092 {
() => {
// Module: crate::value::scalar
// Provides: {"impl_1092"}
// Dependencies: {}
impl < 'a , T : ScalarValue > From < & 'a T > for & 'a Scalar < T > { fn from (value : & 'a T) -> Self { Scalar :: ref_cast (value) } }
};
}
