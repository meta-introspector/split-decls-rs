macro_rules! in_range16x8 {
    () => {
        macro_rules ! in_range16x8 { ($ s : ident , $ start : expr , $ end : expr) => { { ($ s - u16x8 :: splat ($ start)) . simd_lt (u16x8 :: splat ($ end - $ start)) } } ; }
    };
}

in_range16x8!()