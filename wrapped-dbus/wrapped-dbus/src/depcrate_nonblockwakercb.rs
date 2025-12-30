// Generated macro for WakerCb (type)
macro_rules! Depcrate_nonblockWakerCb {
() => {
// Module: crate::nonblock
// Provides: {"WakerCb"}
// Dependencies: {}
# [doc = " Internal callback for the executor when we need wakeup a task"] pub type WakerCb = Box < dyn Fn () -> Result < () , () > + Send + Sync + 'static > ;
};
}
