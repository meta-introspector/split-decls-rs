// Generated macro for impl_311 (impl)
macro_rules! Depcrate_poll_timeoutimpl_311 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_311"}
// Dependencies: {}
impl < T : Into < PollTimeout > > From < Option < T > > for PollTimeout { fn from (x : Option < T >) -> Self { x . map_or (Self :: NONE , | x | x . into ()) } }
};
}
