macro_rules! deps {
    () => {
        I32!();
    };
}

macro_rules! impl_894 {
    () => {
        deps!();
        impl I32 for i32 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i32 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u32 { self as u32 } fn from_bits (n : u32) -> i32 { n as i32 } }
    };
}

impl_894!()