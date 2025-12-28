macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_nopfx_lowercase , false , CheckCase :: Lower) ;
    };
}

macro_72!();