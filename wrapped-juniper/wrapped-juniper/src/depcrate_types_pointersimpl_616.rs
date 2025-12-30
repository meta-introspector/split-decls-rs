// Generated macro for impl_616 (impl)
macro_rules! Depcrate_types_pointersimpl_616 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_616"}
// Dependencies: {}
impl < T , S > ToScalarValue < S > for Arc < T > where T : ToScalarValue < S > + ? Sized , { fn to_scalar_value (& self) -> S { (* * self) . to_scalar_value () } }
};
}
