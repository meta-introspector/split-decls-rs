// Generated macro for impl_320 (impl)
macro_rules! Depcrate_poll_timeoutimpl_320 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_320"}
// Dependencies: {}
impl TryFrom < i32 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i32) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (x)) , } } }
};
}
