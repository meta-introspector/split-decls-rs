// Generated macro for impl_812 (impl)
macro_rules! Depcrate_clientimpl_812 {
() => {
// Module: crate::client
// Provides: {"impl_812"}
// Dependencies: {}
impl PushPromises { # [doc = " Get the next `PushPromise`."] pub async fn push_promise (& mut self) -> Option < Result < PushPromise , crate :: Error > > { crate :: poll_fn (move | cx | self . poll_push_promise (cx)) . await } # [doc (hidden)] pub fn poll_push_promise (& mut self , cx : & mut Context < '_ > ,) -> Poll < Option < Result < PushPromise , crate :: Error > > > { match self . inner . poll_pushed (cx) { Poll :: Ready (Some (Ok ((request , response)))) => { let response = PushedResponseFuture { inner : ResponseFuture { inner : response , push_promise_consumed : false , } , } ; Poll :: Ready (Some (Ok (PushPromise { request , response }))) } Poll :: Ready (Some (Err (e))) => Poll :: Ready (Some (Err (e . into ()))) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
};
}
