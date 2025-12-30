// Generated macro for TryRecvError (enum)
macro_rules! Depcrate_mpscTryRecvError {
() => {
// Module: crate::mpsc
// Provides: {"TryRecvError"}
// Dependencies: {}
# [doc = " Error returned by [`Receiver::try_recv`] or [`UnboundedReceiver::try_recv`]."] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub enum TryRecvError { # [doc = " The channel is empty but not closed."] Empty , # [doc = " The channel is empty and closed."] Closed , }
};
}
