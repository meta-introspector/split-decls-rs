macro_rules! deps {
    () => {
        Error!();
        Result!();
        PollTimeout!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for u64 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
    };
}

impl_262!();