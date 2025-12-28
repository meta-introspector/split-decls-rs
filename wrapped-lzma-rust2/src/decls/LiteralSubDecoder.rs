macro_rules! deps {
    () => {
        LiteralSubCoder!();
    };
}

macro_rules! LiteralSubDecoder {
    () => {
        deps!();
        # [derive (Clone)] struct LiteralSubDecoder { coder : LiteralSubCoder , }
    };
}

LiteralSubDecoder!()