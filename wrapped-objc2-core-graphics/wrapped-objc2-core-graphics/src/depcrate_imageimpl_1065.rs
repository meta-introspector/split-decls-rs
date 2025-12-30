// Generated macro for impl_1065 (impl)
macro_rules! Depcrate_imageimpl_1065 {
() => {
// Module: crate::image
// Provides: {"impl_1065"}
// Dependencies: {}
# [allow (non_upper_case_globals , deprecated)] impl CGImageByteOrderInfo { # [doc (alias = "kCGImageByteOrder16Host")] pub const Order16Host : Self = if cfg ! (target_endian = "big") { Self :: Order16Big } else { Self :: Order16Little } ; # [doc (alias = "kCGImageByteOrder32Host")] pub const Order32Host : Self = if cfg ! (target_endian = "big") { Self :: Order32Big } else { Self :: Order32Little } ; }
};
}
