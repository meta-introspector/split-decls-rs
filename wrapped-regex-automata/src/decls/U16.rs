macro_rules! U16 {
    () => {
        pub (crate) trait U16 { fn as_usize (self) -> usize ; fn low_u8 (self) -> u8 ; fn high_u8 (self) -> u8 ; }
    };
}

U16!();