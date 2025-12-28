macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_withpfx_ignorecase , true , CheckCase :: None) ;
    };
}

macro_69!()