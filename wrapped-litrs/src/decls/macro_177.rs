macro_rules! deps {
    () => {
        ByteStringLit!();
    };
}

macro_rules! macro_177 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , ByteStringLit , ByteString , ByteStringLit) ;
    };
}

macro_177!();