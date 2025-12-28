macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! UNHEX_UPPER {
    () => {
        deps!();
        pub (crate) static UNHEX_UPPER : [u8 ; 256] = init_unhex_array (CheckCase :: Upper) ;
    };
}

UNHEX_UPPER!()