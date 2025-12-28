macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! UNHEX4 {
    () => {
        deps!();
        pub (crate) static UNHEX4 : [u8 ; 256] = init_unhex4_array (CheckCase :: None) ;
    };
}

UNHEX4!()