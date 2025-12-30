// Generated macro for SendError (struct)
macro_rules! Depcrate_mpscSendError {
() => {
// Module: crate::mpsc
// Provides: {"SendError"}
// Dependencies: {}
# [doc = " Error returned when attempting to send after the channels' [Receiver] is dropped or closed."] # [doc = ""] # [doc = " Allows access to message that failed to send with [`into_inner`](Self::into_inner)."] pub struct SendError < T > (pub T) ;
};
}
