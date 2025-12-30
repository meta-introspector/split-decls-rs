// Generated macro for get_memory_usage (function)
macro_rules! Depcrate_lzma_readerget_memory_usage {
() => {
// Module: crate::lzma_reader
// Provides: {"get_memory_usage"}
// Dependencies: {}
# [doc = " Calculates the memory usage in KiB required for LZMA decompression."] pub fn get_memory_usage (dict_size : u32 , lc : u32 , lp : u32) -> crate :: Result < u32 > { if lc > 8 || lp > 4 { return Err (error_invalid_input ("invalid lc or lp")) ; } Ok (10 + get_dict_size (dict_size) ? / 1024 + ((2 * 0x300) << (lc + lp)) / 1024) }
};
}
