macro_rules! deps {
    () => {
        Error!();
        Result!();
        PollTimeoutTryFromError!();
        PollTimeout!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl TryFrom < i16 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i16) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: from (x))) , } } }
    };
}

impl_258!()