macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl From < Hash > for [u8 ; OUT_LEN] { # [inline] fn from (hash : Hash) -> Self { hash . 0 } }
    };
}

impl_38!()