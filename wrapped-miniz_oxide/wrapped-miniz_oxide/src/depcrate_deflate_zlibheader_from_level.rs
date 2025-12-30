// Generated macro for header_from_level (function)
macro_rules! Depcrate_deflate_zlibheader_from_level {
() => {
// Module: crate::deflate::zlib
// Provides: {"header_from_level"}
// Dependencies: {}
# [doc = " Get the zlib header for the level using the default window size and no"] # [doc = " dictionary."] # [inline] fn header_from_level (level : u8 , flags : u32) -> [u8 ; 2] { let cmf = cmf_from_flags (flags) ; [cmf , add_fcheck (cmf , level << 6)] }
};
}
