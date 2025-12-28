macro_rules! deps {
    () => {
        G0!();
    };
}

macro_rules! u32x4x2_generic {
    () => {
        deps!();
        pub type u32x4x2_generic = x2 < u32x4_generic , G0 > ;
    };
}

u32x4x2_generic!();