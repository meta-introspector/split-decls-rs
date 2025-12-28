macro_rules! deps {
    () => {
        Result!();
        PollTimeout!();
        Error!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl TryFrom < u128 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u128) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
    };
}

impl_250!()