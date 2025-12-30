// Generated macro for Teddy (struct)
macro_rules! Depcrate_simd_accel_teddy128Teddy {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"Teddy"}
// Dependencies: {}
# [doc = " A SIMD accelerated multi substring searcher."] # [derive (Debug , Clone)] pub struct Teddy { # [doc = " A list of substrings to match."] pats : Vec < Vec < u8 > > , # [doc = " An Aho-Corasick automaton of the patterns. We use this when we need to"] # [doc = " search pieces smaller than the Teddy block size."] ac : FullAcAutomaton < Vec < u8 > > , # [doc = " A set of 8 buckets. Each bucket corresponds to a single member of a"] # [doc = " bitset. A bucket contains zero or more substrings. This is useful"] # [doc = " when the number of substrings exceeds 8, since our bitsets cannot have"] # [doc = " more than 8 members."] buckets : Vec < Vec < usize > > , # [doc = " Our set of masks. There's one mask for each byte in the fingerprint."] masks : Masks , }
};
}
