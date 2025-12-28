macro_rules! gb2312_level2_hanzi_encode {
    () => {
        # [cfg (not (feature = "fast-gb-hanzi-encode"))] # [inline (always)] pub fn gb2312_level2_hanzi_encode (bmp : u16) -> Option < usize > { position (& GB2312_HANZI [(94 * (0xD8 - 0xB0)) ..] , bmp) }
    };
}

gb2312_level2_hanzi_encode!()