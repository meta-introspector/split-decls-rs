// Generated macro for SendPushedResponse (struct)
macro_rules! Depcrate_serverSendPushedResponse {
() => {
// Module: crate::server
// Provides: {"SendPushedResponse"}
// Dependencies: {}
# [doc = " Send a response to a promised request"] # [doc = ""] # [doc = " A `SendPushedResponse` instance is provided when promising a request and is used"] # [doc = " to send the associated response to the client. It is also used to"] # [doc = " explicitly reset the stream with a custom reason."] # [doc = ""] # [doc = " It can not be used to initiate push promises."] # [doc = ""] # [doc = " If the `SendPushedResponse` instance is dropped without sending a response, then"] # [doc = " the HTTP/2 stream will be reset."] # [doc = ""] # [doc = " See [module] level docs for more details."] # [doc = ""] # [doc = " [module]: index.html"] pub struct SendPushedResponse < B : Buf > { inner : SendResponse < B > , }
};
}
