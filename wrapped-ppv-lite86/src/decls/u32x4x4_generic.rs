macro_rules! u32x4x4_generic {
    () => {
        pub type u32x4x4_generic = x4 < u32x4_generic > ;
    };
}

u32x4x4_generic!();