// Generated macro for decode_surrogate_pair (function)
macro_rules! Depcrate_wtf8decode_surrogate_pair {
() => {
// Module: crate::wtf8
// Provides: {"decode_surrogate_pair"}
// Dependencies: {}
# [inline] fn decode_surrogate_pair (lead : u16 , trail : u16) -> char { let code_point = 0x10000 + ((((lead - 0xD800) as u32) << 10) | (trail - 0xDC00) as u32) ; unsafe { char :: from_u32_unchecked (code_point) } }
};
}
