macro_rules! in_inclusive_range16 {
    () => {
        # [inline (always)] # [cfg (feature = "utf16_iter")] fn in_inclusive_range16 (u : u16 , start : u16 , end : u16) -> bool { u . wrapping_sub (start) <= (end - start) }
    };
}

in_inclusive_range16!()