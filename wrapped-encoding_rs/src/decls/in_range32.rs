macro_rules! in_range32 {
    () => {
        # [inline (always)] fn in_range32 (i : u32 , start : u32 , end : u32) -> bool { i . wrapping_sub (start) < (end - start) }
    };
}

in_range32!();