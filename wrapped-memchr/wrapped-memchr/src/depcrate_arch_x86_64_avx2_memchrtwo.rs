// Generated macro for Two (struct)
macro_rules! Depcrate_arch_x86_64_avx2_memchrTwo {
() => {
// Module: crate::arch::x86_64::avx2::memchr
// Provides: {"Two"}
// Dependencies: {}
# [doc = " Finds all occurrences of two bytes in a haystack."] # [doc = ""] # [doc = " That is, this reports matches of one of two possible bytes. For example,"] # [doc = " searching for `a` or `b` in `afoobar` would report matches at offsets `0`,"] # [doc = " `4` and `5`."] # [derive (Clone , Copy , Debug)] pub struct Two { # [doc = " Used for haystacks less than 32 bytes."] sse2 : generic :: Two < __m128i > , # [doc = " Used for haystacks bigger than 32 bytes."] avx2 : generic :: Two < __m256i > , }
};
}
