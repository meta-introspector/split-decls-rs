// Generated macro for impl_617 (impl)
macro_rules! Depcrate_types_pointersimpl_617 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_617"}
// Dependencies: {}
impl < T , S > FromInputValue < S > for Arc < T > where S : ScalarValue , T : FromInputValue < S > , { type Error = T :: Error ; fn from_input_value (v : & InputValue < S >) -> Result < Arc < T > , Self :: Error > { < T as FromInputValue < S > > :: from_input_value (v) . map (Arc :: new) } }
};
}
