macro_rules! ksx1001_encode_hanja {
    () => {
        # [cfg (feature = "fast-hanja-encode")] # [inline (always)] fn ksx1001_encode_hanja (bmp : u16) -> Option < (u8 , u8) > { if bmp < 0xF900 { ksx1001_unified_hangul_encode (bmp) } else { Some (ksx1001_compatibility_hangul_encode (bmp)) } }
    };
}

ksx1001_encode_hanja!()