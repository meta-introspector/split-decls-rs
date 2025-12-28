macro_rules! I8 {
    () => {
        pub (crate) trait I8 { fn as_usize (self) -> usize ; fn to_bits (self) -> u8 ; fn from_bits (n : u8) -> i8 ; }
    };
}

I8!()