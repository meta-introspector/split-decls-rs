macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl From < PollTimeout > for i64 { fn from (x : PollTimeout) -> Self { Self :: from (x . 0) } }
    };
}

impl_267!()