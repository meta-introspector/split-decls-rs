macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < const N : usize > ByteSlice for [u8 ; N] { # [inline] fn as_bytes (& self) -> & [u8] { self } # [inline] fn as_bytes_mut (& mut self) -> & mut [u8] { self } }
    };
}

impl_61!()