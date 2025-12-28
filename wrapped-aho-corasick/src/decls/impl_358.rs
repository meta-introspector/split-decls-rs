macro_rules! deps {
    () => {
        I8!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl I8 for i8 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i8 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u8 { self as u8 } fn from_bits (n : u8) -> i8 { n as i8 } }
    };
}

impl_358!()