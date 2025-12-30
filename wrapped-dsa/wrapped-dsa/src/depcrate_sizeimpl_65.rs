// Generated macro for impl_65 (impl)
macro_rules! Depcrate_sizeimpl_65 {
() => {
// Module: crate::size
// Provides: {"impl_65"}
// Dependencies: {}
impl KeySize { # [doc = " DSA parameter size constant: L = 1024, N = 160"] # [deprecated (note = "This size constant has a security strength of under 112 bits per SP 800-57 Part 1 Rev. 5")] pub const DSA_1024_160 : Self = Self { l : 1024 , n : 160 } ; # [doc = " DSA parameter size constant: L = 2048, N = 224"] pub const DSA_2048_224 : Self = Self { l : 2048 , n : 224 } ; # [doc = " DSA parameter size constant: L = 2048, N = 256"] pub const DSA_2048_256 : Self = Self { l : 2048 , n : 256 } ; # [doc = " DSA parameter size constant: L = 3072, N = 256"] pub const DSA_3072_256 : Self = Self { l : 3072 , n : 256 } ; # [doc = " Create a KeySize from other, potentially unsafe, key lengths"] # [doc = ""] # [doc = " This aims at supporting non-standard or older/weak keys."] # [cfg (feature = "hazmat")] pub (crate) fn other (l : u32 , n : u32) -> Self { Self { l , n } } }
};
}
