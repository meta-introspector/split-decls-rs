macro_rules! deps {
    () => {
        Error!();
        PollTimeout!();
        Result!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl TryFrom < u64 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u64) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
    };
}

impl_251!()