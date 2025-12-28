macro_rules! deps {
    () => {
        StringLit!();
    };
}

macro_rules! macro_167 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: StringLit < String >, String , StringLit) ;
    };
}

macro_167!();