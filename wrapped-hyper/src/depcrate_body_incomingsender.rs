// Generated macro for Sender (struct)
macro_rules! Depcrate_body_incomingSender {
() => {
// Module: crate::body::incoming
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " A sender half created through [`Body::channel()`]."] # [doc = ""] # [doc = " Useful when wanting to stream chunks from another thread."] # [doc = ""] # [doc = " ## Body Closing"] # [doc = ""] # [doc = " Note that the request body will always be closed normally when the sender is dropped (meaning"] # [doc = " that the empty terminating chunk will be sent to the remote). If you desire to close the"] # [doc = " connection with an incomplete response (e.g. in the case of an error during asynchronous"] # [doc = " processing), call the [`Sender::abort()`] method to abort the body in an abnormal fashion."] # [doc = ""] # [doc = " [`Body::channel()`]: struct.Body.html#method.channel"] # [doc = " [`Sender::abort()`]: struct.Sender.html#method.abort"] # [must_use = "Sender does nothing unless sent on"] # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] pub (crate) struct Sender { want_rx : watch :: Receiver , data_tx : BodySender , trailers_tx : Option < TrailersSender > , }
};
}
