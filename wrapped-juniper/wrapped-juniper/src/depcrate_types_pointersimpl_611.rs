// Generated macro for impl_611 (impl)
macro_rules! Depcrate_types_pointersimpl_611 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_611"}
// Dependencies: {}
impl < T , S > ToInputValue < S > for & T where T : ToInputValue < S > + ? Sized , { fn to_input_value (& self) -> InputValue < S > { (* * self) . to_input_value () } }
};
}
