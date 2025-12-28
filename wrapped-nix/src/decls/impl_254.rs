macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl From < u8 > for PollTimeout { fn from (x : u8) -> Self { Self (i32 :: from (x)) } }
    };
}

impl_254!()