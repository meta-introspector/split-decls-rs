macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        faster_hex_serde_macros ! (nopfx_lowercase , false , CheckCase :: Lower) ;
    };
}

macro_65!();