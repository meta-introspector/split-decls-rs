// Generated macro for get_dict_size (function)
macro_rules! Depcrate_lzma_readerget_dict_size {
() => {
// Module: crate::lzma_reader
// Provides: {"get_dict_size"}
// Dependencies: {}
fn get_dict_size (dict_size : u32) -> crate :: Result < u32 > { if dict_size > DICT_SIZE_MAX { return Err (error_invalid_input ("dict size too large")) ; } let dict_size = dict_size . max (4096) ; Ok ((dict_size + 15) & ! 15) }
};
}
