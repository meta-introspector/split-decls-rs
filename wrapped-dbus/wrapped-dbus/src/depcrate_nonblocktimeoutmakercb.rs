// Generated macro for TimeoutMakerCb (type)
macro_rules! Depcrate_nonblockTimeoutMakerCb {
() => {
// Module: crate::nonblock
// Provides: {"TimeoutMakerCb"}
// Dependencies: {}
# [doc = " Internal callback for the executor when a timeout needs to be made."] pub type TimeoutMakerCb = fn (timeout : Instant) -> pin :: Pin < Box < dyn Future < Output = () > + Send + Sync + 'static > > ;
};
}
