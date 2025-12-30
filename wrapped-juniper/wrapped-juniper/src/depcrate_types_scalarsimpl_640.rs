// Generated macro for impl_640 (impl)
macro_rules! Depcrate_types_scalarsimpl_640 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_640"}
// Dependencies: {}
impl < S : ScalarValue > ToScalarValue < S > for str { fn to_scalar_value (& self) -> S { S :: from_displayable (self) } }
};
}
