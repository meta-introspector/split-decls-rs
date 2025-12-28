macro_rules! deps {
    () => {
        Blob!();
        Value!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl From < Vec < u8 > > for Value { # [inline] fn from (v : Vec < u8 >) -> Self { Self :: Blob (v) } }
    };
}

impl_496!();