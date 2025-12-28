macro_rules! VariantEncoding {
    () => {
        pub enum VariantEncoding { SingleByte (& 'static [u16 ; 128] , u16 , u8 , u8) , Utf8 , Gbk , Gb18030 , Big5 , EucJp , Iso2022Jp , ShiftJis , EucKr , Replacement , Utf16Be , Utf16Le , UserDefined , }
    };
}

VariantEncoding!();