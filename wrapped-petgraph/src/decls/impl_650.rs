macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! impl_650 {
    () => {
        deps!();
        unsafe impl IndexType for usize { # [inline (always)] fn new (x : usize) -> Self { x } # [inline (always)] fn index (& self) -> Self { * self } # [inline (always)] fn max () -> Self { usize :: MAX } }
    };
}

impl_650!()