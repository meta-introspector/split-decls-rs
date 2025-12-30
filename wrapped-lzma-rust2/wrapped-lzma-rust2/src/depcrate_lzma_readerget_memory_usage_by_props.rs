// Generated macro for get_memory_usage_by_props (function)
macro_rules! Depcrate_lzma_readerget_memory_usage_by_props {
() => {
// Module: crate::lzma_reader
// Provides: {"get_memory_usage_by_props"}
// Dependencies: {}
# [doc = " Calculates the memory usage in KiB required for LZMA decompression from properties byte."] pub fn get_memory_usage_by_props (dict_size : u32 , props_byte : u8) -> crate :: Result < u32 > { if dict_size > DICT_SIZE_MAX { return Err (error_invalid_input ("dict size too large")) ; } if props_byte > (4 * 5 + 4) * 9 + 8 { return Err (error_invalid_input ("invalid props byte")) ; } let props = props_byte % (9 * 5) ; let lp = props / 9 ; let lc = props - lp * 9 ; get_memory_usage (dict_size , lc as u32 , lp as u32) }
};
}
