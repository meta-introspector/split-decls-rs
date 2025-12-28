macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! encode_lzma2_dict_size {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn encode_lzma2_dict_size (dict_size : u32) -> crate :: Result < u8 > { if dict_size < 4096 { return Err (error_invalid_input ("LZMA2 dictionary size too small")) ; } if dict_size == 0xFFFFFFFF { return Ok (40) ; } for prop in 0u8 .. 40 { let base = 2 | ((prop & 1) as u32) ; let size = base << (prop / 2 + 11) ; if size >= dict_size { return Ok (prop) ; } } Err (error_invalid_input ("LZMA2 dictionary size too large")) }
    };
}

encode_lzma2_dict_size!();