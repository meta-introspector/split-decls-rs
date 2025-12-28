macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes {
    () => {
        deps!();
        # [apply (all_engines_except_decoder_reader)] fn decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes < E : EngineWrapper > (engine_wrapper : E ,) { for mode in all_pad_modes () { do_invalid_trailing_padding_as_invalid_byte_at_first_padding (E :: standard_with_pad_mode (true , mode) , mode ,) ; } }
    };
}

decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes!();