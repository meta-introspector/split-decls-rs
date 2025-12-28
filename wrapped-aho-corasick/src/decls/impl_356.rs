macro_rules! deps {
    () => {
        U64!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl U64 for u64 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("u64 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn low_u8 (self) -> u8 { self as u8 } fn low_u16 (self) -> u16 { self as u16 } fn low_u32 (self) -> u32 { self as u32 } fn high_u32 (self) -> u32 { (self >> 32) as u32 } }
    };
}

impl_356!();