macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < [u8 ; OUT_LEN] > for Hash { # [inline] fn from (bytes : [u8 ; OUT_LEN]) -> Self { Self :: from_bytes (bytes) } }
    };
}

impl_37!()