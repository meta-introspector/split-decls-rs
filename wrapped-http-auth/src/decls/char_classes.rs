macro_rules! char_classes {
    () => {
        pub (crate) fn char_classes (b : u8) -> u8 { * TABLE . get (usize :: from (b)) . unwrap_or (& 0) }
    };
}

char_classes!();