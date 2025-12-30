// Generated macro for impl_327 (impl)
macro_rules! Depcrate_poll_timeoutimpl_327 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_327"}
// Dependencies: {}
impl TryFrom < PollTimeout > for u16 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
};
}
