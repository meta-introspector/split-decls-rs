macro_rules! deps {
    () => {
        PollTimeout!();
        PollTimeoutTryFromError!();
        Error!();
        Result!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl TryFrom < i128 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i128) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) , } } }
    };
}

impl_255!()