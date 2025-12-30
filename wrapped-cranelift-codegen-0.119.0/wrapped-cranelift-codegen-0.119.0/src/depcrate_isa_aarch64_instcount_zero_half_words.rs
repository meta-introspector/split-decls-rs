// Generated macro for count_zero_half_words (function)
macro_rules! Depcrate_isa_aarch64_instcount_zero_half_words {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"count_zero_half_words"}
// Dependencies: {}
fn count_zero_half_words (mut value : u64 , num_half_words : u8) -> usize { let mut count = 0 ; for _ in 0 .. num_half_words { if value & 0xffff == 0 { count += 1 ; } value >>= 16 ; } count }
};
}
