// Generated macro for SendTimeoutError (enum)
macro_rules! Depcrate_errSendTimeoutError {
() => {
// Module: crate::err
// Provides: {"SendTimeoutError"}
// Dependencies: {}
# [doc = " An error returned from the [`send_timeout`] method."] # [doc = ""] # [doc = " The error contains the message being sent so it can be recovered."] # [doc = ""] # [doc = " [`send_timeout`]: super::Sender::send_timeout"] # [derive (PartialEq , Eq , Clone , Copy)] pub enum SendTimeoutError < T > { # [doc = " The message could not be sent because the channel is full and the operation timed out."] # [doc = ""] # [doc = " If this is a zero-capacity channel, then the error indicates that there was no receiver"] # [doc = " available to receive the message and the operation timed out."] Timeout (T) , # [doc = " The message could not be sent because the channel is disconnected."] Disconnected (T) , }
};
}
