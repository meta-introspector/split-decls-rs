macro_rules! deps {
    () => {
        ByteStringLit!();
    };
}

macro_rules! macro_169 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: ByteStringLit < String >, ByteString , ByteStringLit) ;
    };
}

macro_169!()