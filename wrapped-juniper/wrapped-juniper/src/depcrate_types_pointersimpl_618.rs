// Generated macro for impl_618 (impl)
macro_rules! Depcrate_types_pointersimpl_618 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_618"}
// Dependencies: {}
impl < T , S > ToInputValue < S > for Arc < T > where T : ToInputValue < S > + ? Sized , { fn to_input_value (& self) -> InputValue < S > { (* * self) . to_input_value () } }
};
}
