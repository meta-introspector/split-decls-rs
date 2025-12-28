macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        faster_hex_serde_macros ! (withpfx_ignorecase , true , CheckCase :: None) ;
    };
}

macro_62!()