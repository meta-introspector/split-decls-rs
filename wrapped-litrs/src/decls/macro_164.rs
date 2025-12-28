macro_rules! deps {
    () => {
        IntegerLit!();
    };
}

macro_rules! macro_164 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: IntegerLit < String >, Integer , IntegerLit) ;
    };
}

macro_164!()