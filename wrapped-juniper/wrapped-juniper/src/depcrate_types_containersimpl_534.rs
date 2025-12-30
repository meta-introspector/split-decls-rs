// Generated macro for impl_534 (impl)
macro_rules! Depcrate_types_containersimpl_534 {
() => {
// Module: crate::types::containers
// Provides: {"impl_534"}
// Dependencies: {}
impl < T , S > IntoFieldError < S > for FromInputValueArrayError < T , S > where T : FromInputValue < S > , T :: Error : IntoFieldError < S > , S : ScalarValue , { fn into_field_error (self) -> FieldError < S > { const ERROR_PREFIX : & str = "Failed to convert into exact-size array" ; match self { Self :: Null => format ! ("{ERROR_PREFIX}: Value cannot be `null`") . into () , Self :: WrongCount { actual , expected } => { format ! ("{ERROR_PREFIX}: wrong elements count: {actual} instead of {expected}" ,) . into () } Self :: Item (s) => s . into_field_error () , } } }
};
}
