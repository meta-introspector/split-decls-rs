macro_rules! deps {
    () => {
        LiteralCoder!();
        LiteralSubDecoder!();
    };
}

macro_rules! LiteralDecoder {
    () => {
        deps!();
        pub (crate) struct LiteralDecoder { coder : LiteralCoder , sub_decoders : Vec < LiteralSubDecoder > , }
    };
}

LiteralDecoder!();