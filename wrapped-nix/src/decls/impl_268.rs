macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl From < PollTimeout > for i32 { fn from (x : PollTimeout) -> Self { x . 0 } }
    };
}

impl_268!()