macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! macro_64 {
    () => {
        deps!();
        faster_hex_serde_macros ! (withpfx_lowercase , true , CheckCase :: Lower) ;
    };
}

macro_64!()