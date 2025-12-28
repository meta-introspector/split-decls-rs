macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl From < u16 > for PollTimeout { fn from (x : u16) -> Self { Self (i32 :: from (x)) } }
    };
}

impl_253!()