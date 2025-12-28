macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_70 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_nopfx_ignorecase , false , CheckCase :: None) ;
    };
}

macro_70!()