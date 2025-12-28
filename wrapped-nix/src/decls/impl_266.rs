macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl From < PollTimeout > for i128 { fn from (x : PollTimeout) -> Self { Self :: from (x . 0) } }
    };
}

impl_266!()