macro_rules! deps {
    () => {
        ShiftJisEncoder!();
        Utf8Encoder!();
        Iso2022JpEncoder!();
        EucKrEncoder!();
        Gb18030Encoder!();
        EucJpEncoder!();
        UserDefinedEncoder!();
        Big5Encoder!();
        SingleByteEncoder!();
    };
}

macro_rules! VariantEncoder {
    () => {
        deps!();
        pub enum VariantEncoder { SingleByte (SingleByteEncoder) , Utf8 (Utf8Encoder) , Gb18030 (Gb18030Encoder) , Big5 (Big5Encoder) , EucJp (EucJpEncoder) , Iso2022Jp (Iso2022JpEncoder) , ShiftJis (ShiftJisEncoder) , EucKr (EucKrEncoder) , UserDefined (UserDefinedEncoder) , }
    };
}

VariantEncoder!();