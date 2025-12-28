macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        unsafe impl IndexType for u8 { # [inline (always)] fn new (x : usize) -> Self { x as u8 } # [inline (always)] fn index (& self) -> usize { * self as usize } # [inline (always)] fn max () -> Self { u8 :: MAX } }
    };
}

impl_653!()