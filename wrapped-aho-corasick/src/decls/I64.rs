macro_rules! I64 {
    () => {
        pub (crate) trait I64 { fn as_usize (self) -> usize ; fn to_bits (self) -> u64 ; fn from_bits (n : u64) -> i64 ; }
    };
}

I64!()