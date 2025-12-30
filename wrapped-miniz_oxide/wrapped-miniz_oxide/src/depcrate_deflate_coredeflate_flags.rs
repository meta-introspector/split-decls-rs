// Generated macro for deflate_flags (module)
macro_rules! Depcrate_deflate_coredeflate_flags {
() => {
// Module: crate::deflate::core
// Provides: {"deflate_flags"}
// Dependencies: {}
pub mod deflate_flags { # [doc = " Whether to use a zlib wrapper."] pub const TDEFL_WRITE_ZLIB_HEADER : u32 = 0x0000_1000 ; # [doc = " Should we compute the adler32 checksum."] pub const TDEFL_COMPUTE_ADLER32 : u32 = 0x0000_2000 ; # [doc = " Should we use greedy parsing (as opposed to lazy parsing where look ahead one or more"] # [doc = " bytes to check for better matches.)"] pub const TDEFL_GREEDY_PARSING_FLAG : u32 = 0x0000_4000 ; # [doc = " Used in miniz to skip zero-initializing hash and dict. We don't do this here, so"] # [doc = " this flag is ignored."] pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG : u32 = 0x0000_8000 ; # [doc = " Only look for matches with a distance of 0."] pub const TDEFL_RLE_MATCHES : u32 = 0x0001_0000 ; # [doc = " Only use matches that are at least 6 bytes long."] pub const TDEFL_FILTER_MATCHES : u32 = 0x0002_0000 ; # [doc = " Force the compressor to only output static blocks. (Blocks using the default huffman codes"] # [doc = " specified in the deflate specification.)"] pub const TDEFL_FORCE_ALL_STATIC_BLOCKS : u32 = 0x0004_0000 ; # [doc = " Force the compressor to only output raw/uncompressed blocks."] pub const TDEFL_FORCE_ALL_RAW_BLOCKS : u32 = 0x0008_0000 ; }
};
}
