macro_rules! deps {
    () => {
        LiteralSubCoder!();
    };
}

macro_rules! LiteralSubEncoder {
    () => {
        deps!();
        # [derive (Clone)] struct LiteralSubEncoder { coder : LiteralSubCoder , }
    };
}

LiteralSubEncoder!();