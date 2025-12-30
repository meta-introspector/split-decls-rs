// Generated macro for SendRequest (struct)
macro_rules! Depcrate_clientSendRequest {
() => {
// Module: crate::client
// Provides: {"SendRequest"}
// Dependencies: {}
# [doc = " Initializes new HTTP/2 streams on a connection by sending a request."] # [doc = ""] # [doc = " This type does no work itself. Instead, it is a handle to the inner"] # [doc = " connection state held by [`Connection`]. If the associated connection"] # [doc = " instance is dropped, all `SendRequest` functions will return [`Error`]."] # [doc = ""] # [doc = " [`SendRequest`] instances are able to move to and operate on separate tasks"] # [doc = " / threads than their associated [`Connection`] instance. Internally, there"] # [doc = " is a buffer used to stage requests before they get written to the"] # [doc = " connection. There is no guarantee that requests get written to the"] # [doc = " connection in FIFO order as HTTP/2 prioritization logic can play a role."] # [doc = ""] # [doc = " [`SendRequest`] implements [`Clone`], enabling the creation of many"] # [doc = " instances that are backed by a single connection."] # [doc = ""] # [doc = " See [module] level documentation for more details."] # [doc = ""] # [doc = " [module]: index.html"] # [doc = " [`Connection`]: struct.Connection.html"] # [doc = " [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html"] # [doc = " [`Error`]: ../struct.Error.html"] pub struct SendRequest < B : Buf > { inner : proto :: Streams < B , Peer > , pending : Option < proto :: OpaqueStreamRef > , }
};
}
