// Generated macro for Three (struct)
macro_rules! Depcrate_arch_x86_64_avx2_memchrThree {
() => {
// Module: crate::arch::x86_64::avx2::memchr
// Provides: {"Three"}
// Dependencies: {}
# [doc = " Finds all occurrences of three bytes in a haystack."] # [doc = ""] # [doc = " That is, this reports matches of one of three possible bytes. For example,"] # [doc = " searching for `a`, `b` or `o` in `afoobar` would report matches at offsets"] # [doc = " `0`, `2`, `3`, `4` and `5`."] # [derive (Clone , Copy , Debug)] pub struct Three { # [doc = " Used for haystacks less than 32 bytes."] sse2 : generic :: Three < __m128i > , # [doc = " Used for haystacks bigger than 32 bytes."] avx2 : generic :: Three < __m256i > , }
};
}
