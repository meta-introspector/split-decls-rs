macro_rules! deps {
    () => {
        I64!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl I64 for i64 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i64 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u64 { self as u64 } fn from_bits (n : u64) -> i64 { n as i64 } }
    };
}

impl_362!()