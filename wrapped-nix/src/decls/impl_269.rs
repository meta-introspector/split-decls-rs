macro_rules! deps {
    () => {
        PollTimeout!();
        Result!();
        Error!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for i16 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
    };
}

impl_269!();