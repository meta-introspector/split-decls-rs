macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        faster_hex_serde_macros ! (nopfx_ignorecase , false , CheckCase :: None) ;
    };
}

macro_63!()