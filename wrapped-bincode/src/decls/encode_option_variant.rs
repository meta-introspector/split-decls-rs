macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! encode_option_variant {
    () => {
        deps!();
        # [doc = " Encode the variant of the given option. Will not encode the option itself."] # [inline] pub (crate) fn encode_option_variant < E : Encoder , T > (encoder : & mut E , value : & Option < T > ,) -> Result < () , EncodeError > { match value { None => 0u8 . encode (encoder) , Some (_) => 1u8 . encode (encoder) , } }
    };
}

encode_option_variant!();