macro_rules! MH_NOMULTIDEFS {
    () => {
        # [doc = " this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used."] pub const MH_NOMULTIDEFS : u32 = 0x200 ;
    };
}

MH_NOMULTIDEFS!()