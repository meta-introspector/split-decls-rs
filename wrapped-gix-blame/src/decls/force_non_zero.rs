macro_rules! force_non_zero {
    () => {
        fn force_non_zero (n : u32) -> NonZeroU32 { NonZeroU32 :: new (n) . expect ("BUG: hunks are never empty") }
    };
}

force_non_zero!()