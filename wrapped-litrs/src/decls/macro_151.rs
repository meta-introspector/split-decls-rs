macro_rules! deps {
    () => {
        IntegerLit!();
    };
}

macro_rules! macro_151 {
    () => {
        deps!();
        impl_specific_lit_to_lit ! (crate :: IntegerLit < B >, Integer) ;
    };
}

macro_151!()