macro_rules! deps {
    () => {
        U16!();
    };
}

macro_rules! impl_888 {
    () => {
        deps!();
        impl U16 for u16 { fn as_usize (self) -> usize { usize :: from (self) } fn low_u8 (self) -> u8 { self as u8 } fn high_u8 (self) -> u8 { (self >> 8) as u8 } }
    };
}

impl_888!();