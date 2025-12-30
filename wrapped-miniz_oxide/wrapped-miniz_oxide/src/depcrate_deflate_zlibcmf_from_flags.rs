// Generated macro for cmf_from_flags (function)
macro_rules! Depcrate_deflate_zlibcmf_from_flags {
() => {
// Module: crate::deflate::zlib
// Provides: {"cmf_from_flags"}
// Dependencies: {}
# [inline] const fn cmf_from_flags (flags : u32) -> u8 { if (flags & TDEFL_RLE_MATCHES == 0) && (flags & TDEFL_FORCE_ALL_RAW_BLOCKS == 0) { DEFAULT_CMF } else { MIN_CMF } }
};
}
