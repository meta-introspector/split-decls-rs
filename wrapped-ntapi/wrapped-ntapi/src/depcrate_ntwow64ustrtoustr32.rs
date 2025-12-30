// Generated macro for UStrToUStr32 (function)
macro_rules! Depcrate_ntwow64UStrToUStr32 {
() => {
// Module: crate::ntwow64
// Provides: {"UStrToUStr32"}
// Dependencies: {}
# [inline] pub fn UStrToUStr32 (Destination : & mut UNICODE_STRING32 , Source : & UNICODE_STRING ,) { Destination . Length = Source . Length ; Destination . MaximumLength = Source . MaximumLength ; Destination . Buffer = Source . Buffer as u32 ; }
};
}
