macro_rules! deps {
    () => {
        GeneralPurposeConfig!();
    };
}

macro_rules! PAD {
    () => {
        deps!();
        # [doc = " Include padding bytes when encoding, and require that they be present when decoding."] # [doc = ""] # [doc = " This is the standard per the base64 RFC, but consider using [`NO_PAD`] or [`NO_PAD_INDIFFERENT`]"] # [doc = " instead as padding serves little purpose in practice."] pub const PAD : GeneralPurposeConfig = GeneralPurposeConfig :: new () ;
    };
}

PAD!()