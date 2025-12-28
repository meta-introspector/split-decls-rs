macro_rules! RangeDecoder {
    () => {
        pub (crate) struct RangeDecoder < R > { inner : R , range : u32 , code : u32 , }
    };
}

RangeDecoder!()