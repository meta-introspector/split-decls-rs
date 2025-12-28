macro_rules! deps {
    () => {
        ByteLit!();
    };
}

macro_rules! macro_155 {
    () => {
        deps!();
        impl_specific_lit_to_lit ! (crate :: ByteLit < B >, Byte) ;
    };
}

macro_155!();