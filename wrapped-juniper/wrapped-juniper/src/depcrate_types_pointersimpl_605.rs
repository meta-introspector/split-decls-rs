// Generated macro for impl_605 (impl)
macro_rules! Depcrate_types_pointersimpl_605 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_605"}
// Dependencies: {}
impl < T , S > FromInputValue < S > for Box < T > where S : ScalarValue , T : FromInputValue < S > , { type Error = T :: Error ; fn from_input_value (v : & InputValue < S >) -> Result < Box < T > , Self :: Error > { < T as FromInputValue < S > > :: from_input_value (v) . map (Box :: new) } }
};
}
