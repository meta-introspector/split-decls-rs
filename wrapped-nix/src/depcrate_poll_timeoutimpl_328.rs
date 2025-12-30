// Generated macro for impl_328 (impl)
macro_rules! Depcrate_poll_timeoutimpl_328 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_328"}
// Dependencies: {}
impl TryFrom < PollTimeout > for u8 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
};
}
