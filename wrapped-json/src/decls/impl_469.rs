macro_rules! deps {
    () => {
        Mantissa!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl Mantissa for u64 { const HIMASK : u64 = 0xFFFFFFFF00000000 ; const LOMASK : u64 = 0x00000000FFFFFFFF ; const FULL : i32 = 64 ; }
    };
}

impl_469!()