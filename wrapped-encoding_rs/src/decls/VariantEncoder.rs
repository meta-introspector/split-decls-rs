macro_rules! deps {
    () => {
        Gb18030Encoder!();
        SingleByteEncoder!();
        ShiftJisEncoder!();
        EucKrEncoder!();
        UserDefinedEncoder!();
        EucJpEncoder!();
        Iso2022JpEncoder!();
        Utf8Encoder!();
        Big5Encoder!();
    };
}

macro_rules! VariantEncoder {
    () => {
        deps!();
        pub enum VariantEncoder { SingleByte (SingleByteEncoder) , Utf8 (Utf8Encoder) , Gb18030 (Gb18030Encoder) , Big5 (Big5Encoder) , EucJp (EucJpEncoder) , Iso2022Jp (Iso2022JpEncoder) , ShiftJis (ShiftJisEncoder) , EucKr (EucKrEncoder) , UserDefined (UserDefinedEncoder) , }
    };
}

VariantEncoder!()