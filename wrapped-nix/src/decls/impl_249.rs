macro_rules! deps {
    () => {
        PollTimeout!();
        Error!();
        Result!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl TryFrom < Duration > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : Duration) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x . as_millis ()) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
    };
}

impl_249!()