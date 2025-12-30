// Generated macro for impl_814 (impl)
macro_rules! Depcrate_clientimpl_814 {
() => {
// Module: crate::client
// Provides: {"impl_814"}
// Dependencies: {}
impl PushPromise { # [doc = " Returns a reference to the push promise's request headers."] pub fn request (& self) -> & Request < () > { & self . request } # [doc = " Returns a mutable reference to the push promise's request headers."] pub fn request_mut (& mut self) -> & mut Request < () > { & mut self . request } # [doc = " Consumes `self`, returning the push promise's request headers and"] # [doc = " response future."] pub fn into_parts (self) -> (Request < () > , PushedResponseFuture) { (self . request , self . response) } }
};
}
