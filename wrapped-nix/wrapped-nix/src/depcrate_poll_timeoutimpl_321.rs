// Generated macro for impl_321 (impl)
macro_rules! Depcrate_poll_timeoutimpl_321 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_321"}
// Dependencies: {}
impl TryFrom < i16 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i16) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: from (x))) , } } }
};
}
