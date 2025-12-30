// Generated macro for TrySendError (enum)
macro_rules! DepcrateTrySendError {
() => {
// Module: crate
// Provides: {"TrySendError"}
// Dependencies: {}
# [doc = " An error returned from [`Sender::try_send()`]."] # [derive (PartialEq , Eq , Clone , Copy)] pub enum TrySendError < T > { # [doc = " The channel is full but not closed."] Full (T) , # [doc = " The channel is closed."] Closed (T) , }
};
}
