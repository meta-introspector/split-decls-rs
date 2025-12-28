macro_rules! in_inclusive_range8 {
    () => {
        # [inline (always)] fn in_inclusive_range8 (i : u8 , start : u8 , end : u8) -> bool { i . wrapping_sub (start) <= (end - start) }
    };
}

in_inclusive_range8!()