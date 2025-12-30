// Generated macro for One (struct)
macro_rules! Depcrate_arch_x86_64_avx2_memchrOne {
() => {
// Module: crate::arch::x86_64::avx2::memchr
// Provides: {"One"}
// Dependencies: {}
# [doc = " Finds all occurrences of a single byte in a haystack."] # [derive (Clone , Copy , Debug)] pub struct One { # [doc = " Used for haystacks less than 32 bytes."] sse2 : generic :: One < __m128i > , # [doc = " Used for haystacks bigger than 32 bytes."] avx2 : generic :: One < __m256i > , }
};
}
