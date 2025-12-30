// Generated macro for impl_61 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_61 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_61"}
// Dependencies: {}
impl Error { # [doc = " Returns true if this was an error from `Connect`."] pub fn is_connect (& self) -> bool { matches ! (self . kind , ErrorKind :: Connect) } # [doc = " Returns the info of the client connection on which this error occurred."] # [cfg (any (feature = "http1" , feature = "http2"))] pub fn connect_info (& self) -> Option < & Connected > { self . connect_info . as_ref () } # [cfg (any (feature = "http1" , feature = "http2"))] fn with_connect_info (self , connect_info : Connected) -> Self { Self { connect_info : Some (connect_info) , .. self } } fn is_canceled (& self) -> bool { matches ! (self . kind , ErrorKind :: Canceled) } fn tx (src : hyper :: Error) -> Self { e ! (SendRequest , src) } fn closed (src : hyper :: Error) -> Self { e ! (ChannelClosed , src) } }
};
}
