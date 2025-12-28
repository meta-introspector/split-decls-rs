macro_rules! deps {
    () => {
        Iso2022JpDecoderState!();
    };
}

macro_rules! Iso2022JpDecoder {
    () => {
        deps!();
        pub struct Iso2022JpDecoder { decoder_state : Iso2022JpDecoderState , output_state : Iso2022JpDecoderState , lead : u8 , output_flag : bool , pending_prepended : bool , }
    };
}

Iso2022JpDecoder!();