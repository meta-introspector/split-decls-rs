macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_all_modes {
    () => {
        deps!();
        # [doc = " Any amount of padding anywhere before the final non padding character = invalid byte at first"] # [doc = " pad byte."] # [doc = " From this and [decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_non_canonical_padding_suffix_all_modes],"] # [doc = " we know padding must extend contiguously to the end of the input."] # [apply (all_engines)] fn decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_all_modes < E : EngineWrapper , > (engine_wrapper : E ,) { let suffixes = & [("AA==" , 2) , ("AAA=" , 1) , ("AAAA" , 0)] ; for mode in pad_modes_allowing_padding () { let engine = E :: standard_with_pad_mode (true , mode) ; decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad (engine , suffixes . as_slice () ,) ; } }
    };
}

decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_all_modes!()