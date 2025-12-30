// Generated macro for impl_318 (impl)
macro_rules! Depcrate_poll_timeoutimpl_318 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_318"}
// Dependencies: {}
impl TryFrom < i128 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i128) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) , } } }
};
}
