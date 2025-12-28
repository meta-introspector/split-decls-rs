macro_rules! U8 {
    () => {
        pub (crate) trait U8 { fn as_usize (self) -> usize ; }
    };
}

U8!();