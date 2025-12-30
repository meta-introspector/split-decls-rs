// Generated macro for UStr32ToUStr (function)
macro_rules! Depcrate_ntwow64UStr32ToUStr {
() => {
// Module: crate::ntwow64
// Provides: {"UStr32ToUStr"}
// Dependencies: {}
# [inline] pub fn UStr32ToUStr (Destination : & mut UNICODE_STRING , Source : & UNICODE_STRING32 ,) { Destination . Length = Source . Length ; Destination . MaximumLength = Source . MaximumLength ; Destination . Buffer = Source . Buffer as * mut u16 ; }
};
}
