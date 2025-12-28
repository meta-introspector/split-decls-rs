macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl From < Hash > for [u8 ; OUT_LEN] { # [inline] fn from (hash : Hash) -> Self { hash . 0 } }
    };
}

impl_171!()