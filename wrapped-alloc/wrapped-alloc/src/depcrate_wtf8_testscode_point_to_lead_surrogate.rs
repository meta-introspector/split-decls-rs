// Generated macro for code_point_to_lead_surrogate (function)
macro_rules! Depcrate_wtf8_testscode_point_to_lead_surrogate {
() => {
// Module: crate::wtf8::tests
// Provides: {"code_point_to_lead_surrogate"}
// Dependencies: {}
# [test] fn code_point_to_lead_surrogate () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } assert_eq ! (c (0) . to_lead_surrogate () , None) ; assert_eq ! (c (0xE9) . to_lead_surrogate () , None) ; assert_eq ! (c (0xD800) . to_lead_surrogate () , Some (0xD800)) ; assert_eq ! (c (0xDBFF) . to_lead_surrogate () , Some (0xDBFF)) ; assert_eq ! (c (0xDC00) . to_lead_surrogate () , None) ; assert_eq ! (c (0xDFFF) . to_lead_surrogate () , None) ; assert_eq ! (c (0x1F4A9) . to_lead_surrogate () , None) ; assert_eq ! (c (0x10FFFF) . to_lead_surrogate () , None) ; }
};
}
