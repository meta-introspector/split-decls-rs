// Generated macro for impl_313 (impl)
macro_rules! Depcrate_poll_timeoutimpl_313 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_313"}
// Dependencies: {}
impl TryFrom < u128 > for PollTimeout { type Error = PollTimeoutTryFromError ; fn try_from (x : u128) -> std :: result :: Result < Self , Self :: Error > { Ok (Self (i32 :: try_from (x) . map_err (| _ | PollTimeoutTryFromError :: TooPositive) ? ,)) } }
};
}
