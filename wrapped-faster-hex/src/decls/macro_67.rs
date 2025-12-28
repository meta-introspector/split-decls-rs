macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_67 {
    () => {
        deps!();
        faster_hex_serde_macros ! (nopfx_uppercase , false , CheckCase :: Upper) ;
    };
}

macro_67!();