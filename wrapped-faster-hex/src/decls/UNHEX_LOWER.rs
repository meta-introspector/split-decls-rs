macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! UNHEX_LOWER {
    () => {
        deps!();
        pub (crate) static UNHEX_LOWER : [u8 ; 256] = init_unhex_array (CheckCase :: Lower) ;
    };
}

UNHEX_LOWER!();