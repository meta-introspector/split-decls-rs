macro_rules! u128x4_generic {
    () => {
        pub type u128x4_generic = x4 < u128x1_generic > ;
    };
}

u128x4_generic!()