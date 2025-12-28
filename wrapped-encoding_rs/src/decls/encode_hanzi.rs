macro_rules! encode_hanzi {
    () => {
        # [cfg (feature = "fast-gb-hanzi-encode")] # [inline (always)] fn encode_hanzi (_ : u16 , bmp_minus_unified_start : u16) -> (u8 , u8) { gbk_hanzi_encode (bmp_minus_unified_start) }
    };
}

encode_hanzi!();