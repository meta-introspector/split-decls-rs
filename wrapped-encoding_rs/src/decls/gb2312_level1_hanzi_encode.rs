macro_rules! gb2312_level1_hanzi_encode {
    () => {
        # [cfg (all (feature = "less-slow-gb-hanzi-encode" , not (feature = "fast-gb-hanzi-encode")))] # [inline (always)] pub fn gb2312_level1_hanzi_encode (bmp : u16) -> Option < (u8 , u8) > { match GB2312_LEVEL1_HANZI_CODE_POINTS . binary_search (& bmp) { Ok (i) => { let pair = & GB2312_LEVEL1_HANZI_BYTES [i] ; Some ((pair [0] , pair [1])) } Err (_) => None , } }
    };
}

gb2312_level1_hanzi_encode!();