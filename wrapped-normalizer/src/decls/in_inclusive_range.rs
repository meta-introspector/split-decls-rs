macro_rules! in_inclusive_range {
    () => {
        # [inline (always)] fn in_inclusive_range (c : char , start : char , end : char) -> bool { u32 :: from (c) . wrapping_sub (u32 :: from (start)) <= (u32 :: from (end) - u32 :: from (start)) }
    };
}

in_inclusive_range!()