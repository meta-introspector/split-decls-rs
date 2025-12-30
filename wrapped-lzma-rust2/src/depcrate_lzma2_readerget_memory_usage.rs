// Generated macro for get_memory_usage (function)
macro_rules! Depcrate_lzma2_readerget_memory_usage {
() => {
// Module: crate::lzma2_reader
// Provides: {"get_memory_usage"}
// Dependencies: {}
# [doc = " Calculates the memory usage in KiB required for LZMA2 decompression."] # [inline] pub fn get_memory_usage (dict_size : u32) -> u32 { 40 + COMPRESSED_SIZE_MAX / 1024 + get_dict_size (dict_size) / 1024 }
};
}
