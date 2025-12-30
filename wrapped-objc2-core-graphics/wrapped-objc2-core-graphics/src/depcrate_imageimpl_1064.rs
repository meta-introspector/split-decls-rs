// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_imageimpl_1064 {
() => {
// Module: crate::image
// Provides: {"impl_1064"}
// Dependencies: {}
# [allow (non_upper_case_globals , deprecated)] impl CGBitmapInfo { # [doc (alias = "kCGBitmapByteOrder16Host")] pub const ByteOrder16Host : Self = if cfg ! (target_endian = "big") { Self :: ByteOrder16Big } else { Self :: ByteOrder16Little } ; # [doc (alias = "kCGBitmapByteOrder32Host")] pub const ByteOrder32Host : Self = if cfg ! (target_endian = "big") { Self :: ByteOrder32Big } else { Self :: ByteOrder32Little } ; }
};
}
