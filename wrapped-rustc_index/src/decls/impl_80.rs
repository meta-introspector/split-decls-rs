macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl Idx for u32 { # [inline] fn new (idx : usize) -> Self { assert ! (idx <= u32 :: MAX as usize) ; idx as u32 } # [inline] fn index (self) -> usize { self as usize } }
    };
}

impl_80!();