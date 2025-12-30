// Generated macro for impl_603 (impl)
macro_rules! Depcrate_types_pointersimpl_603 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_603"}
// Dependencies: {}
impl < 's , T , S > FromScalarValue < 's , S > for Box < T > where S : ScalarValue , T : FromScalarValue < 's , S > + 's , { type Error = T :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { T :: from_scalar_value (v) . map (Self :: new) } }
};
}
