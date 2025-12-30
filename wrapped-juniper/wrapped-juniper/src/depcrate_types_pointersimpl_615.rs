// Generated macro for impl_615 (impl)
macro_rules! Depcrate_types_pointersimpl_615 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_615"}
// Dependencies: {}
impl < 's , T , S > FromScalarValue < 's , S > for Arc < T > where S : ScalarValue , T : FromScalarValue < 's , S > + 's , { type Error = T :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { T :: from_scalar_value (v) . map (Self :: new) } }
};
}
