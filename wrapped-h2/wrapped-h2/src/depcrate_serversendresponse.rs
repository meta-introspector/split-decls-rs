// Generated macro for SendResponse (struct)
macro_rules! Depcrate_serverSendResponse {
() => {
// Module: crate::server
// Provides: {"SendResponse"}
// Dependencies: {}
# [doc = " Send a response back to the client"] # [doc = ""] # [doc = " A `SendResponse` instance is provided when receiving a request and is used"] # [doc = " to send the associated response back to the client. It is also used to"] # [doc = " explicitly reset the stream with a custom reason."] # [doc = ""] # [doc = " It will also be used to initiate push promises linked with the associated"] # [doc = " stream."] # [doc = ""] # [doc = " If the `SendResponse` instance is dropped without sending a response, then"] # [doc = " the HTTP/2 stream will be reset."] # [doc = ""] # [doc = " See [module] level docs for more details."] # [doc = ""] # [doc = " [module]: index.html"] # [derive (Debug)] pub struct SendResponse < B : Buf > { inner : proto :: StreamRef < B > , }
};
}
