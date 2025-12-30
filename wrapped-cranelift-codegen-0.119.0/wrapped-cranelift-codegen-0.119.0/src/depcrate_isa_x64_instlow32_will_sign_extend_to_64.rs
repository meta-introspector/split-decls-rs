// Generated macro for low32_will_sign_extend_to_64 (function)
macro_rules! Depcrate_isa_x64_instlow32_will_sign_extend_to_64 {
() => {
// Module: crate::isa::x64::inst
// Provides: {"low32_will_sign_extend_to_64"}
// Dependencies: {}
pub (crate) fn low32_will_sign_extend_to_64 (x : u64) -> bool { let xs = x as i64 ; xs == ((xs << 32) >> 32) }
};
}
