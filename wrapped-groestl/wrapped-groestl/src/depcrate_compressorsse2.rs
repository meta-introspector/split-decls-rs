// Generated macro for sse2 (module)
macro_rules! Depcrate_compressorsse2 {
() => {
// Module: crate::compressor
// Provides: {"sse2"}
// Dependencies: {}
# [cfg (not (target_feature = "ssse3"))] # [cfg (any (feature = "std" , target_feature = "sse2"))] pub mod sse2 { use super :: * ; # [target_feature (enable = "sse2")] pub unsafe fn tf512 (cv : & mut X4 , data : * const u8) { tf512_impl (cv , data) } # [target_feature (enable = "sse2")] pub unsafe fn of512 (cv : & mut X4) { of512_impl (cv) } # [target_feature (enable = "sse2")] pub unsafe fn init512 (cv : X4) -> X4 { init512_impl (cv) } # [target_feature (enable = "sse2")] pub unsafe fn tf1024 (cv : & mut X8 , data : * const u8) { tf1024_impl (cv , data) } # [target_feature (enable = "sse2")] pub unsafe fn of1024 (cv : & mut X8) { of1024_impl (cv) } # [target_feature (enable = "sse2")] pub unsafe fn init1024 (cv : X8) -> X8 { init1024_impl (cv) } }
};
}
