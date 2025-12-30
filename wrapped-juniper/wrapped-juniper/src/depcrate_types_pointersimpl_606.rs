// Generated macro for impl_606 (impl)
macro_rules! Depcrate_types_pointersimpl_606 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_606"}
// Dependencies: {}
impl < T , S > ToInputValue < S > for Box < T > where T : ToInputValue < S > + ? Sized , { fn to_input_value (& self) -> InputValue < S > { (* * self) . to_input_value () } }
};
}
