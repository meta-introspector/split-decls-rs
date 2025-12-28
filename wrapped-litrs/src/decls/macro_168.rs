macro_rules! deps {
    () => {
        ByteLit!();
    };
}

macro_rules! macro_168 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: ByteLit < String >, Byte , ByteLit) ;
    };
}

macro_168!()