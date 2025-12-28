macro_rules! deps {
    () => {
        Result!();
        PollTimeout!();
        Error!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl TryFrom < i64 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i64) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) , } } }
    };
}

impl_256!()