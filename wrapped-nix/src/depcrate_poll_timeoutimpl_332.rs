// Generated macro for impl_332 (impl)
macro_rules! Depcrate_poll_timeoutimpl_332 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_332"}
// Dependencies: {}
impl TryFrom < PollTimeout > for i16 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
};
}
