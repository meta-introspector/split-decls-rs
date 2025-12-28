macro_rules! get_extra_size_before {
    () => {
        # [doc = " Calculates the extra space needed before the dictionary for LZMA2 encoding."] pub fn get_extra_size_before (dict_size : u32) -> u32 { COMPRESSED_SIZE_MAX . saturating_sub (dict_size) }
    };
}

get_extra_size_before!()