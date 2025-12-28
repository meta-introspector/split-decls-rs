macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_invalid_trailing_bytes_invalid_byte {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_invalid_trailing_bytes_invalid_byte < E : EngineWrapper > (engine_wrapper : E) { for mode in pad_modes_allowing_padding () { do_invalid_trailing_byte (E :: standard_with_pad_mode (true , mode) , mode) ; } }
    };
}

decode_invalid_trailing_bytes_invalid_byte!()