macro_rules! deps {
    () => {
        G1!();
    };
}

macro_rules! u64x4_generic {
    () => {
        deps!();
        pub type u64x4_generic = x2 < u64x2_generic , G1 > ;
    };
}

u64x4_generic!()