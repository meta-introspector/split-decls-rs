macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! UNHEX {
    () => {
        deps!();
        pub (crate) static UNHEX : [u8 ; 256] = init_unhex_array (CheckCase :: None) ;
    };
}

UNHEX!()