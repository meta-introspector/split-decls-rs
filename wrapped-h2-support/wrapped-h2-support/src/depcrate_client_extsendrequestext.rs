// Generated macro for SendRequestExt (trait)
macro_rules! Depcrate_client_extSendRequestExt {
() => {
// Module: crate::client_ext
// Provides: {"SendRequestExt"}
// Dependencies: {}
# [doc = " Extend the `h2::client::SendRequest` type with convenience methods."] pub trait SendRequestExt { # [doc = " Convenience method to send a GET request and ignore the SendStream"] # [doc = " (since GETs don't need to send a body)."] fn get (& mut self , uri : & str) -> ResponseFuture ; }
};
}
