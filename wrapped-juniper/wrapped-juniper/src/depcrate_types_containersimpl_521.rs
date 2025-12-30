// Generated macro for impl_521 (impl)
macro_rules! Depcrate_types_containersimpl_521 {
() => {
// Module: crate::types::containers
// Provides: {"impl_521"}
// Dependencies: {}
impl < T , S > ToInputValue < S > for Vec < T > where T : ToInputValue < S > , { fn to_input_value (& self) -> InputValue < S > { InputValue :: list (self . iter () . map (T :: to_input_value) . collect ()) } }
};
}
