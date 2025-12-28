macro_rules! char_from_u16 {
    () => {
        # [doc = " Convert a `u16` _obtained from data provider data_ to `char`."] # [inline (always)] fn char_from_u16 (u : u16) -> char { char_from_u32 (u32 :: from (u)) }
    };
}

char_from_u16!()