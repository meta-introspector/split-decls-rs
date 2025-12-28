macro_rules! deps {
    () => {
        Gb18030Decoder!();
        EucKrEncoder!();
        SingleByteDecoder!();
        SingleByteEncoder!();
        EucKrDecoder!();
        Big5Encoder!();
        Iso2022JpEncoder!();
        ShiftJisDecoder!();
        Gb18030Encoder!();
        ShiftJisEncoder!();
        Utf16Decoder!();
        Utf8Encoder!();
        ReplacementDecoder!();
        Utf8Decoder!();
        Encoder!();
        EucJpEncoder!();
        Encoding!();
        Iso2022JpDecoder!();
        VariantDecoder!();
        UserDefinedEncoder!();
        EucJpDecoder!();
        Big5Decoder!();
        UserDefinedDecoder!();
        VariantEncoding!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl VariantEncoding { pub fn new_variant_decoder (& self) -> VariantDecoder { match * self { VariantEncoding :: SingleByte (table , _ , _ , _) => SingleByteDecoder :: new (table) , VariantEncoding :: Utf8 => Utf8Decoder :: new () , VariantEncoding :: Gbk | VariantEncoding :: Gb18030 => Gb18030Decoder :: new () , VariantEncoding :: Big5 => Big5Decoder :: new () , VariantEncoding :: EucJp => EucJpDecoder :: new () , VariantEncoding :: Iso2022Jp => Iso2022JpDecoder :: new () , VariantEncoding :: ShiftJis => ShiftJisDecoder :: new () , VariantEncoding :: EucKr => EucKrDecoder :: new () , VariantEncoding :: Replacement => ReplacementDecoder :: new () , VariantEncoding :: UserDefined => UserDefinedDecoder :: new () , VariantEncoding :: Utf16Be => Utf16Decoder :: new (true) , VariantEncoding :: Utf16Le => Utf16Decoder :: new (false) , } } pub fn new_encoder (& self , encoding : & 'static Encoding) -> Encoder { match * self { VariantEncoding :: SingleByte (table , run_bmp_offset , run_byte_offset , run_length) => { SingleByteEncoder :: new (encoding , table , run_bmp_offset , run_byte_offset , run_length) } VariantEncoding :: Utf8 => Utf8Encoder :: new (encoding) , VariantEncoding :: Gbk => Gb18030Encoder :: new (encoding , false) , VariantEncoding :: Gb18030 => Gb18030Encoder :: new (encoding , true) , VariantEncoding :: Big5 => Big5Encoder :: new (encoding) , VariantEncoding :: EucJp => EucJpEncoder :: new (encoding) , VariantEncoding :: Iso2022Jp => Iso2022JpEncoder :: new (encoding) , VariantEncoding :: ShiftJis => ShiftJisEncoder :: new (encoding) , VariantEncoding :: EucKr => EucKrEncoder :: new (encoding) , VariantEncoding :: UserDefined => UserDefinedEncoder :: new (encoding) , VariantEncoding :: Utf16Be | VariantEncoding :: Replacement | VariantEncoding :: Utf16Le => { unreachable ! () } } } pub fn is_single_byte (& self) -> bool { match * self { VariantEncoding :: SingleByte (_ , _ , _ , _) | VariantEncoding :: UserDefined => true , _ => false , } } }
    };
}

impl_360!();