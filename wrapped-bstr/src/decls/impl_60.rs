macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl ByteSlice for [u8] { # [inline] fn as_bytes (& self) -> & [u8] { self } # [inline] fn as_bytes_mut (& mut self) -> & mut [u8] { self } }
    };
}

impl_60!();