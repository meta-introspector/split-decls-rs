macro_rules! char_from_u32 {
    () => {
        # [doc = " Convert a `u32` _obtained from data provider data_ to `char`."] # [inline (always)] fn char_from_u32 (u : u32) -> char { unwrap_or_gigo (core :: char :: from_u32 (u) , REPLACEMENT_CHARACTER) }
    };
}

char_from_u32!()