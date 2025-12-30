// Generated macro for encode_lzma2_dict_size (function)
macro_rules! Depcrate_xzencode_lzma2_dict_size {
() => {
// Module: crate::xz
// Provides: {"encode_lzma2_dict_size"}
// Dependencies: {}
# [cfg (feature = "encoder")] fn encode_lzma2_dict_size (dict_size : u32) -> crate :: Result < u8 > { if dict_size < 4096 { return Err (error_invalid_input ("LZMA2 dictionary size too small")) ; } if dict_size == 0xFFFFFFFF { return Ok (40) ; } for prop in 0u8 .. 40 { let base = 2 | ((prop & 1) as u32) ; let size = base << (prop / 2 + 11) ; if size >= dict_size { return Ok (prop) ; } } Err (error_invalid_input ("LZMA2 dictionary size too large")) }
};
}
