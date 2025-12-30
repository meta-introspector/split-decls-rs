// Generated macro for impl_312 (impl)
macro_rules! Depcrate_poll_timeoutimpl_312 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_312"}
// Dependencies: {}
impl TryFrom < Duration > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : Duration) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x . as_millis ()) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
};
}
