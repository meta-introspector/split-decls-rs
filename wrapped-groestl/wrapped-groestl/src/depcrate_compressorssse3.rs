// Generated macro for ssse3 (module)
macro_rules! Depcrate_compressorssse3 {
() => {
// Module: crate::compressor
// Provides: {"ssse3"}
// Dependencies: {}
# [cfg (not (target_feature = "aes"))] # [cfg (any (feature = "std" , target_feature = "ssse3"))] pub mod ssse3 { use super :: * ; # [target_feature (enable = "sse2" , enable = "ssse3")] pub unsafe fn tf512 (cv : & mut X4 , data : * const u8) { tf512_impl (cv , data) } # [target_feature (enable = "sse2" , enable = "ssse3")] pub unsafe fn of512 (cv : & mut X4) { of512_impl (cv) } # [target_feature (enable = "sse2" , enable = "ssse3")] pub unsafe fn tf1024 (cv : & mut X8 , data : * const u8) { tf1024_impl (cv , data) } # [target_feature (enable = "sse2" , enable = "ssse3")] pub unsafe fn of1024 (cv : & mut X8) { of1024_impl (cv) } pub use super :: aes :: { init1024 , init512 } ; }
};
}
