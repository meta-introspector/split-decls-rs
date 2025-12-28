macro_rules! deps {
    () => {
        Result!();
        PollTimeout!();
        Error!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for u32 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
    };
}

impl_263!();