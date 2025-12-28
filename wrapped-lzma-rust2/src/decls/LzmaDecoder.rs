macro_rules! deps {
    () => {
        LzmaCoder!();
        LiteralDecoder!();
        LengthCoder!();
    };
}

macro_rules! LzmaDecoder {
    () => {
        deps!();
        pub (crate) struct LzmaDecoder { coder : LzmaCoder , literal_decoder : LiteralDecoder , match_len_decoder : LengthCoder , rep_len_decoder : LengthCoder , }
    };
}

LzmaDecoder!();