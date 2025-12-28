macro_rules! deps {
    () => {
        DecodePaddingMode!();
        GeneralPurposeConfig!();
    };
}

macro_rules! NO_PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " Don't add padding when encoding, and allow input with or without padding when decoding."] pub const NO_PAD_INDIFFERENT : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (false) . with_decode_padding_mode (DecodePaddingMode :: Indifferent) ;
    };
}

NO_PAD_INDIFFERENT!()