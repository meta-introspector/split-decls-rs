macro_rules! deps {
    () => {
        BoolLit!();
    };
}

macro_rules! macro_150 {
    () => {
        deps!();
        impl_specific_lit_to_lit ! (crate :: BoolLit , Bool) ;
    };
}

macro_150!();