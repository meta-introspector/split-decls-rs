// Generated macro for RtlInitEmptyUnicodeString (function)
macro_rules! Depcrate_ntrtlRtlInitEmptyUnicodeString {
() => {
// Module: crate::ntrtl
// Provides: {"RtlInitEmptyUnicodeString"}
// Dependencies: {}
# [inline] pub fn RtlInitEmptyUnicodeString (UnicodeString : & mut UNICODE_STRING , Buffer : PWCHAR , MaximumLength : USHORT ,) { UnicodeString . Buffer = Buffer ; UnicodeString . MaximumLength = MaximumLength ; UnicodeString . Length = 0 ; }
};
}
