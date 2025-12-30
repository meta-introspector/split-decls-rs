// Generated macro for impl_322 (impl)
macro_rules! Depcrate_poll_timeoutimpl_322 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_322"}
// Dependencies: {}
impl TryFrom < i8 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : i8) -> std :: result :: Result < Self , Self :: Error > { match x { ..= - 2 => Err (PollTimeoutTryFromError :: TooNegative) , - 1 .. => Ok (Self (i32 :: from (x))) , } } }
};
}
