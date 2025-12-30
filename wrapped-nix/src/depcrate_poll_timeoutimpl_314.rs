// Generated macro for impl_314 (impl)
macro_rules! Depcrate_poll_timeoutimpl_314 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_314"}
// Dependencies: {}
impl TryFrom < u64 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u64) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
};
}
