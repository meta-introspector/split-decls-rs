macro_rules! deps {
    () => {
        GeneralPurposeConfig!();
        DecodePaddingMode!();
    };
}

macro_rules! NO_PAD {
    () => {
        deps!();
        # [doc = " Don't add padding when encoding, and require that there is no padding when decoding."] pub const NO_PAD : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (false) . with_decode_padding_mode (DecodePaddingMode :: RequireNone) ;
    };
}

NO_PAD!()