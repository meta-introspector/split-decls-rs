// Generated macro for swap16_s2 (function)
macro_rules! Depcrate_x86_64_sse2swap16_s2 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"swap16_s2"}
// Dependencies: {}
# [inline (always)] fn swap16_s2 (x : __m128i) -> __m128i { unsafe { _mm_shufflehi_epi16 (_mm_shufflelo_epi16 (x , 0b1011_0001) , 0b1011_0001) } }
};
}
