// Generated macro for impl_532 (impl)
macro_rules! Depcrate_types_containersimpl_532 {
() => {
// Module: crate::types::containers
// Provides: {"impl_532"}
// Dependencies: {}
impl < T , S , const N : usize > ToInputValue < S > for [T ; N] where T : ToInputValue < S > , { fn to_input_value (& self) -> InputValue < S > { InputValue :: list (self . iter () . map (T :: to_input_value) . collect ()) } }
};
}
