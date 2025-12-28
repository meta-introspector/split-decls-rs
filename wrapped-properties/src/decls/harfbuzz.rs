macro_rules! harfbuzz {
    () => {
        # [cfg (feature = "harfbuzz_traits")] mod harfbuzz ;
    };
}

harfbuzz!();