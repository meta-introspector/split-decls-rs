macro_rules! in_range16 {
    () => {
        # [inline (always)] fn in_range16 (i : u16 , start : u16 , end : u16) -> bool { i . wrapping_sub (start) < (end - start) }
    };
}

in_range16!()