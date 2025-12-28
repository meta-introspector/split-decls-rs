macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        faster_hex_serde_option_macros ! (option_withpfx_uppercase , true , CheckCase :: Upper) ;
    };
}

macro_73!()