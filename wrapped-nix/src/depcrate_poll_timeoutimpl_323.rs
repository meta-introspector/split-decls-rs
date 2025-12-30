// Generated macro for impl_323 (impl)
macro_rules! Depcrate_poll_timeoutimpl_323 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_323"}
// Dependencies: {}
impl TryFrom < PollTimeout > for Duration { type Error = () ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , () > { x . duration () . ok_or (()) } }
};
}
