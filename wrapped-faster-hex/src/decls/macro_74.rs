macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_nopfx_uppercase , false , CheckCase :: Upper) ;
    };
}

macro_74!();