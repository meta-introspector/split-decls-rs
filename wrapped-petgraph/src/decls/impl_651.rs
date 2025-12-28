macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        unsafe impl IndexType for u32 { # [inline (always)] fn new (x : usize) -> Self { x as u32 } # [inline (always)] fn index (& self) -> usize { * self as usize } # [inline (always)] fn max () -> Self { u32 :: MAX } }
    };
}

impl_651!()