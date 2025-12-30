// Generated macro for impl_527 (impl)
macro_rules! Depcrate_types_containersimpl_527 {
() => {
// Module: crate::types::containers
// Provides: {"impl_527"}
// Dependencies: {}
impl < T , S > ToInputValue < S > for [T] where T : ToInputValue < S > , { fn to_input_value (& self) -> InputValue < S > { InputValue :: list (self . iter () . map (T :: to_input_value) . collect ()) } }
};
}
