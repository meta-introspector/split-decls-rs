macro_rules! deps {
    () => {
        ByteStringLit!();
    };
}

macro_rules! macro_156 {
    () => {
        deps!();
        impl_specific_lit_to_lit ! (crate :: ByteStringLit < B >, ByteString) ;
    };
}

macro_156!()