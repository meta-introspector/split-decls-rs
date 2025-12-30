// Generated macro for impl_315 (impl)
macro_rules! Depcrate_poll_timeoutimpl_315 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_315"}
// Dependencies: {}
impl TryFrom < u32 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u32) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
};
}
