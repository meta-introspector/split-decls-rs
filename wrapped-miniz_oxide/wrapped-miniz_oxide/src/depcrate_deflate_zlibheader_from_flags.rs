// Generated macro for header_from_flags (function)
macro_rules! Depcrate_deflate_zlibheader_from_flags {
() => {
// Module: crate::deflate::zlib
// Provides: {"header_from_flags"}
// Dependencies: {}
# [doc = " Create a zlib header from the given compression flags."] # [doc = " Only level is considered."] # [inline] pub fn header_from_flags (flags : u32) -> [u8 ; 2] { let level = zlib_level_from_flags (flags) ; header_from_level (level , flags) }
};
}
