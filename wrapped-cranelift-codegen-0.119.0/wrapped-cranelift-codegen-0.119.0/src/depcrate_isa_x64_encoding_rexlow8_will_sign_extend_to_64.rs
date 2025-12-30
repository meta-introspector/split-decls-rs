// Generated macro for low8_will_sign_extend_to_64 (function)
macro_rules! Depcrate_isa_x64_encoding_rexlow8_will_sign_extend_to_64 {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"low8_will_sign_extend_to_64"}
// Dependencies: {}
pub (crate) fn low8_will_sign_extend_to_64 (x : u32) -> bool { let xs = (x as i32) as i64 ; xs == ((xs << 56) >> 56) }
};
}
