macro_rules! deps {
    () => {
        FloatLit!();
    };
}

macro_rules! macro_165 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: FloatLit < String >, Float , FloatLit) ;
    };
}

macro_165!();