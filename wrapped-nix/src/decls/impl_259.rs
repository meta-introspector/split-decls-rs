macro_rules! deps {
    () => {
        PollTimeout!();
        Error!();
        Result!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl TryFrom < i8 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i8) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: from (x))) , } } }
    };
}

impl_259!();