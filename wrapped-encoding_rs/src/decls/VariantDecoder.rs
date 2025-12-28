macro_rules! deps {
    () => {
        ReplacementDecoder!();
        UserDefinedDecoder!();
        SingleByteDecoder!();
        Utf16Decoder!();
        ShiftJisDecoder!();
        Gb18030Decoder!();
        EucKrDecoder!();
        EucJpDecoder!();
        Iso2022JpDecoder!();
        Big5Decoder!();
        Utf8Decoder!();
    };
}

macro_rules! VariantDecoder {
    () => {
        deps!();
        pub enum VariantDecoder { SingleByte (SingleByteDecoder) , Utf8 (Utf8Decoder) , Gb18030 (Gb18030Decoder) , Big5 (Big5Decoder) , EucJp (EucJpDecoder) , Iso2022Jp (Iso2022JpDecoder) , ShiftJis (ShiftJisDecoder) , EucKr (EucKrDecoder) , Replacement (ReplacementDecoder) , UserDefined (UserDefinedDecoder) , Utf16 (Utf16Decoder) , }
    };
}

VariantDecoder!();