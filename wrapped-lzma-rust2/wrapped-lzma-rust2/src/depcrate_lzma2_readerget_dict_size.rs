// Generated macro for get_dict_size (function)
macro_rules! Depcrate_lzma2_readerget_dict_size {
() => {
// Module: crate::lzma2_reader
// Provides: {"get_dict_size"}
// Dependencies: {}
# [inline] fn get_dict_size (dict_size : u32) -> u32 { if dict_size >= (u32 :: MAX - 15) { return u32 :: MAX ; } (dict_size + 15) & ! 15 }
};
}
