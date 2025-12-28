macro_rules! deps {
    () => {
        G0!();
    };
}

macro_rules! u64x2x2_generic {
    () => {
        deps!();
        pub type u64x2x2_generic = x2 < u64x2_generic , G0 > ;
    };
}

u64x2x2_generic!()