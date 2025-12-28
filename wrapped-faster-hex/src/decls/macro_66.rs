macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_66 {
    () => {
        deps!();
        faster_hex_serde_macros ! (withpfx_uppercase , true , CheckCase :: Upper) ;
    };
}

macro_66!();