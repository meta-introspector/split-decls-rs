macro_rules! deps {
    () => {
        LengthEncoder!();
        LzmaEncData!();
        LzmaCoder!();
        LiteralEncoder!();
        LzEncoder!();
    };
}

macro_rules! LzmaEncoder {
    () => {
        deps!();
        pub (crate) struct LzmaEncoder { pub (crate) coder : LzmaCoder , pub (crate) lz : LzEncoder , pub (crate) literal_encoder : LiteralEncoder , pub (crate) match_len_encoder : LengthEncoder , pub (crate) rep_len_encoder : LengthEncoder , pub (crate) data : LzmaEncData , }
    };
}

LzmaEncoder!()