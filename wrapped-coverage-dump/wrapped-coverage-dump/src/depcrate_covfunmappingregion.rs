// Generated macro for MappingRegion (struct)
macro_rules! Depcrate_covfunMappingRegion {
() => {
// Module: crate::covfun
// Provides: {"MappingRegion"}
// Dependencies: {}
struct MappingRegion { # [doc = " Offset of this region's start line, relative to the *start line* of"] # [doc = " the *previous mapping* (or 0). Line numbers are 1-based."] start_line_offset : u32 , # [doc = " This region's start column, absolute and 1-based."] start_column : u32 , # [doc = " Offset of this region's end line, relative to the *this mapping's*"] # [doc = " start line. Line numbers are 1-based."] end_line_offset : u32 , # [doc = " This region's end column, absolute, 1-based, and exclusive."] # [doc = ""] # [doc = " If the highest bit is set, that bit is cleared and the associated"] # [doc = " mapping becomes a gap region mapping."] end_column : u32 , }
};
}
