// Generated macro for SendError (struct)
macro_rules! DepcrateSendError {
() => {
// Module: crate
// Provides: {"SendError"}
// Dependencies: {}
# [doc = " An error returned from [`Sender::send()`]."] # [doc = ""] # [doc = " Received because the channel is closed."] # [derive (PartialEq , Eq , Clone , Copy)] pub struct SendError < T > (pub T) ;
};
}
