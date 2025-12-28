macro_rules! u64x2x4_generic {
    () => {
        pub type u64x2x4_generic = x4 < u64x2_generic > ;
    };
}

u64x2x4_generic!();