macro_rules! deps {
    () => {
        HashValue!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl HashValue { # [inline (always)] fn get (self) -> u64 { self . 0 as u64 } }
    };
}

impl_188!()