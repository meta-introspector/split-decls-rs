macro_rules! gbk_hanzi_encode {
    () => {
        # [cfg (feature = "fast-gb-hanzi-encode")] # [inline (always)] pub fn gbk_hanzi_encode (bmp_minus_start : u16) -> (u8 , u8) { let pair = & GBK_HANZI_BYTES [bmp_minus_start as usize] ; (pair [0] , pair [1]) }
    };
}

gbk_hanzi_encode!()