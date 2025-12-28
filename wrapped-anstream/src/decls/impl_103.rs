macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Buffer { # [inline] pub fn new () -> Self { Default :: default () } # [inline] pub fn with_capacity (capacity : usize) -> Self { Self (Vec :: with_capacity (capacity)) } # [inline] pub fn as_bytes (& self) -> & [u8] { & self . 0 } }
    };
}

impl_103!();