// Generated macro for SendError (struct)
macro_rules! Depcrate_errSendError {
() => {
// Module: crate::err
// Provides: {"SendError"}
// Dependencies: {}
# [doc = " An error returned from the [`send`] method."] # [doc = ""] # [doc = " The message could not be sent because the channel is disconnected."] # [doc = ""] # [doc = " The error contains the message so it can be recovered."] # [doc = ""] # [doc = " [`send`]: super::Sender::send"] # [derive (PartialEq , Eq , Clone , Copy)] pub struct SendError < T > (pub T) ;
};
}
