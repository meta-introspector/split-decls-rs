macro_rules! deps {
    () => {
        CStringLit!();
    };
}

macro_rules! macro_170 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: CStringLit < String >, CString , CStringLit) ;
    };
}

macro_170!()