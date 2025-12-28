macro_rules! deps {
    () => {
        PollTimeout!();
        Error!();
        Result!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for i8 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
    };
}

impl_270!();