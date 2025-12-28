macro_rules! RangeEncoder {
    () => {
        pub (crate) struct RangeEncoder < W > { low : u64 , range : u32 , cache_size : u32 , cache : u8 , inner : W , }
    };
}

RangeEncoder!();