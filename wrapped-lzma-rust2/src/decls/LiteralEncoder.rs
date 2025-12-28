macro_rules! deps {
    () => {
        LiteralCoder!();
        LiteralSubEncoder!();
    };
}

macro_rules! LiteralEncoder {
    () => {
        deps!();
        pub (crate) struct LiteralEncoder { coder : LiteralCoder , sub_encoders : Vec < LiteralSubEncoder > , }
    };
}

LiteralEncoder!();