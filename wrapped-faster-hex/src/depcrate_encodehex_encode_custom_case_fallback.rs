// Generated macro for hex_encode_custom_case_fallback (function)
macro_rules! Depcrate_encodehex_encode_custom_case_fallback {
() => {
// Module: crate::encode
// Provides: {"hex_encode_custom_case_fallback"}
// Dependencies: {}
fn hex_encode_custom_case_fallback (src : & [u8] , dst : & mut [u8] , upper_case : bool) { if upper_case { for (byte , slots) in src . iter () . zip (dst . chunks_exact_mut (2)) { slots [0] = hex_upper ((* byte >> 4) & 0xf) ; slots [1] = hex_upper (* byte & 0xf) ; } } else { for (byte , slots) in src . iter () . zip (dst . chunks_exact_mut (2)) { slots [0] = hex_lower ((* byte >> 4) & 0xf) ; slots [1] = hex_lower (* byte & 0xf) ; } } }
};
}
