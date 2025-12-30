// Generated macro for impl_523 (impl)
macro_rules! Depcrate_types_containersimpl_523 {
() => {
// Module: crate::types::containers
// Provides: {"impl_523"}
// Dependencies: {}
impl < T , S > IntoFieldError < S > for FromInputValueVecError < T , S > where T : FromInputValue < S > , T :: Error : IntoFieldError < S > , S : ScalarValue , { fn into_field_error (self) -> FieldError < S > { match self { Self :: Null => "Failed to convert into `Vec`: Value cannot be `null`" . into () , Self :: Item (s) => s . into_field_error () , } } }
};
}
