macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_withpfx_lowercase , true , CheckCase :: Lower) ;
    };
}

macro_71!()