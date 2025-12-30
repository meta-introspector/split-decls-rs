// Generated macro for impl_326 (impl)
macro_rules! Depcrate_poll_timeoutimpl_326 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_326"}
// Dependencies: {}
impl TryFrom < PollTimeout > for u32 { type Error = < Self as TryFrom < i32 > > :: Error ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , Self :: Error > { Self :: try_from (x . 0) } }
};
}
