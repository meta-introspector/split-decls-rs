// Generated macro for impl_610 (impl)
macro_rules! Depcrate_types_pointersimpl_610 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_610"}
// Dependencies: {}
impl < T , S > ToScalarValue < S > for & T where T : ToScalarValue < S > + ? Sized , { fn to_scalar_value (& self) -> S { (* * self) . to_scalar_value () } }
};
}
