macro_rules! deps {
    () => {
        Gb18030Decoder!();
        EucJpDecoder!();
        SingleByteDecoder!();
        Big5Decoder!();
        UserDefinedDecoder!();
        EucKrDecoder!();
        ShiftJisDecoder!();
        Utf8Decoder!();
        ReplacementDecoder!();
        Utf16Decoder!();
        Iso2022JpDecoder!();
    };
}

macro_rules! VariantDecoder {
    () => {
        deps!();
        pub enum VariantDecoder { SingleByte (SingleByteDecoder) , Utf8 (Utf8Decoder) , Gb18030 (Gb18030Decoder) , Big5 (Big5Decoder) , EucJp (EucJpDecoder) , Iso2022Jp (Iso2022JpDecoder) , ShiftJis (ShiftJisDecoder) , EucKr (EucKrDecoder) , Replacement (ReplacementDecoder) , UserDefined (UserDefinedDecoder) , Utf16 (Utf16Decoder) , }
    };
}

VariantDecoder!()