macro_rules! deps {
    () => {
        DecodePaddingMode!();
        GeneralPurposeConfig!();
    };
}

macro_rules! PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " Include padding bytes when encoding, but allow input with or without padding when decoding."] pub const PAD_INDIFFERENT : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (true) . with_decode_padding_mode (DecodePaddingMode :: Indifferent) ;
    };
}

PAD_INDIFFERENT!()