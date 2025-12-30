// Generated macro for impl_309 (impl)
macro_rules! Depcrate_poll_timeoutimpl_309 {
() => {
// Module: crate::poll_timeout
// Provides: {"impl_309"}
// Dependencies: {}
impl std :: fmt :: Display for PollTimeoutTryFromError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: TooNegative => write ! (f , "Passed a negative timeout less than -1.") , Self :: TooPositive => write ! (f , "Passed a positive timeout greater than `i32::MAX` milliseconds.") } } }
};
}
