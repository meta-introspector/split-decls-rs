macro_rules! deps {
    () => {
        Error!();
        PollTimeout!();
        Result!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for u128 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
    };
}

impl_261!()