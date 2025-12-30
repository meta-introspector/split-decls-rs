// Generated macro for low8_will_sign_extend_to_32 (function)
macro_rules! Depcrate_isa_x64_encoding_rexlow8_will_sign_extend_to_32 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"low8_will_sign_extend_to_32"}
// Dependencies: {}
pub (crate) fn low8_will_sign_extend_to_32 (x : u32) -> bool { let xs = x as i32 ; xs == ((xs << 24) >> 24) }
};
}
