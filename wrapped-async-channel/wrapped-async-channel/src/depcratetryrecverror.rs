// Generated macro for TryRecvError (enum)
macro_rules! DepcrateTryRecvError {
() => {
// Module: crate
// Provides: {"TryRecvError"}
// Dependencies: {}
# [doc = " An error returned from [`Receiver::try_recv()`]."] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub enum TryRecvError { # [doc = " The channel is empty but not closed."] Empty , # [doc = " The channel is empty and closed."] Closed , }
};
}
