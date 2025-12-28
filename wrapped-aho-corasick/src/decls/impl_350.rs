macro_rules! deps {
    () => {
        U8!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl U8 for u8 { fn as_usize (self) -> usize { usize :: from (self) } }
    };
}

impl_350!();