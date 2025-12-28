macro_rules! deps {
    () => {
        PollTimeout!();
        PollTimeoutTryFromError!();
        Error!();
        Result!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl TryFrom < u32 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u32) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
    };
}

impl_252!()