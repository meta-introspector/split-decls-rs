// Generated macro for RtlIsNullOrEmptyUnicodeString (function)
macro_rules! Depcrate_ntrtlRtlIsNullOrEmptyUnicodeString {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsNullOrEmptyUnicodeString"}
// Dependencies: {}
# [inline] pub unsafe fn RtlIsNullOrEmptyUnicodeString (String : PUNICODE_STRING) -> bool { String . is_null () || (* String) . Length == 0 }
};
}
