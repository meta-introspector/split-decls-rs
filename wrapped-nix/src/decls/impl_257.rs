macro_rules! deps {
    () => {
        PollTimeout!();
        Result!();
        PollTimeoutTryFromError!();
        Error!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl TryFrom < i32 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i32) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (x)) , } } }
    };
}

impl_257!()