// Generated macro for RecvStream (struct)
macro_rules! Depcrate_shareRecvStream {
() => {
// Module: crate::share
// Provides: {"RecvStream"}
// Dependencies: {}
# [doc = " Receives the body stream and trailers from the remote peer."] # [doc = ""] # [doc = " A `RecvStream` is provided by [`client::ResponseFuture`] and"] # [doc = " [`server::Connection`] with the received HTTP/2 message head (the response"] # [doc = " and request head respectively)."] # [doc = ""] # [doc = " A `RecvStream` instance is used to receive the streaming message body and"] # [doc = " any trailers from the remote peer. It is also used to manage inbound flow"] # [doc = " control."] # [doc = ""] # [doc = " See method level documentation for more details on receiving data. See"] # [doc = " [`FlowControl`] for more details on inbound flow control."] # [doc = ""] # [doc = " [`client::ResponseFuture`]: client/struct.ResponseFuture.html"] # [doc = " [`server::Connection`]: server/struct.Connection.html"] # [doc = " [`FlowControl`]: struct.FlowControl.html"] # [doc = " [`Stream`]: https://docs.rs/futures/0.1/futures/stream/trait.Stream.html"] # [must_use = "streams do nothing unless polled"] pub struct RecvStream { inner : FlowControl , }
};
}
