macro_rules! deps {
    () => {
        DecodePaddingMode!();
    };
}

macro_rules! NaiveConfig {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] pub struct NaiveConfig { pub encode_padding : bool , pub decode_allow_trailing_bits : bool , pub decode_padding_mode : DecodePaddingMode , }
    };
}

NaiveConfig!()