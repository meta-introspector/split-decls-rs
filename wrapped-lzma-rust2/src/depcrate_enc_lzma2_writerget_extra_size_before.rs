// Generated macro for get_extra_size_before (function)
macro_rules! Depcrate_enc_lzma2_writerget_extra_size_before {
() => {
// Module: crate::enc::lzma2_writer
// Provides: {"get_extra_size_before"}
// Dependencies: {}
# [doc = " Calculates the extra space needed before the dictionary for LZMA2 encoding."] pub fn get_extra_size_before (dict_size : u32) -> u32 { COMPRESSED_SIZE_MAX . saturating_sub (dict_size) }
};
}
