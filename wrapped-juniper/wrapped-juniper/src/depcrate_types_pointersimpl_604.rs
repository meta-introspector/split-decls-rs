// Generated macro for impl_604 (impl)
macro_rules! Depcrate_types_pointersimpl_604 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_604"}
// Dependencies: {}
impl < T , S > ToScalarValue < S > for Box < T > where T : ToScalarValue < S > + ? Sized , { fn to_scalar_value (& self) -> S { (* * self) . to_scalar_value () } }
};
}
