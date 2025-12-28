macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_invalid_trailing_padding_as_invalid_byte_at_first_pad_byte {
    () => {
        deps!();
        # [doc = " When there's 1 trailing byte, but it's padding, it's only InvalidByte if there isn't padding"] # [doc = " earlier."] # [apply (all_engines)] fn decode_invalid_trailing_padding_as_invalid_byte_at_first_pad_byte < E : EngineWrapper > (engine_wrapper : E ,) { for mode in pad_modes_allowing_padding () { do_invalid_trailing_padding_as_invalid_byte_at_first_padding (E :: standard_with_pad_mode (true , mode) , mode ,) ; } }
    };
}

decode_invalid_trailing_padding_as_invalid_byte_at_first_pad_byte!()