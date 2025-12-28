macro_rules! deps {
    () => {
        G0!();
    };
}

macro_rules! u128x2_generic {
    () => {
        deps!();
        pub type u128x2_generic = x2 < u128x1_generic , G0 > ;
    };
}

u128x2_generic!()