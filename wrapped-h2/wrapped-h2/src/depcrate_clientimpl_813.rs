// Generated macro for impl_813 (impl)
macro_rules! Depcrate_clientimpl_813 {
() => {
// Module: crate::client
// Provides: {"impl_813"}
// Dependencies: {}
# [cfg (feature = "stream")] impl futures_core :: Stream for PushPromises { type Item = Result < PushPromise , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_push_promise (cx) } }
};
}
