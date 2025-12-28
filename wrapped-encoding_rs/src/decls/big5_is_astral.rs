macro_rules! big5_is_astral {
    () => {
        # [inline (always)] pub fn big5_is_astral (rebased_pointer : usize) -> bool { (BIG5_ASTRALNESS [rebased_pointer >> 5] & (1 << (rebased_pointer & 0x1F))) != 0 }
    };
}

big5_is_astral!()