macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        unsafe impl IndexType for u16 { # [inline (always)] fn new (x : usize) -> Self { x as u16 } # [inline (always)] fn index (& self) -> usize { * self as usize } # [inline (always)] fn max () -> Self { u16 :: MAX } }
    };
}

impl_652!();