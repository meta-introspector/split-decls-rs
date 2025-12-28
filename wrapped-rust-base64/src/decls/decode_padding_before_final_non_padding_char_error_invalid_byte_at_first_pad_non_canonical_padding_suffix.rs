macro_rules! deps {
    () => {
        DecodePaddingMode!();
        EngineWrapper!();
    };
}

macro_rules! decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_non_canonical_padding_suffix {
    () => {
        deps!();
        # [doc = " See [decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_all_modes]"] # [apply (all_engines)] fn decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_non_canonical_padding_suffix < E : EngineWrapper , > (engine_wrapper : E ,) { let suffixes = [("AA==" , 2) , ("AA=" , 1) , ("AA" , 0) , ("AAA=" , 1) , ("AAA" , 0) , ("AAAA" , 0) ,] ; let engine = E :: standard_with_pad_mode (true , DecodePaddingMode :: Indifferent) ; decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad (engine , suffixes . as_slice () ,) }
    };
}

decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad_non_canonical_padding_suffix!();