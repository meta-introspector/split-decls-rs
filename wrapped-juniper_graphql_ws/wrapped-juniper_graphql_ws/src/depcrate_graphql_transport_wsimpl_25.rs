// Generated macro for impl_25 (impl)
macro_rules! Depcrate_graphql_transport_wsimpl_25 {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"impl_25"}
// Dependencies: {}
impl < S : ScalarValue + Send > Output < S > { # [doc = " Converts the reaction into a one-item stream."] fn into_stream (self) -> BoxStream < 'static , Self > { stream :: once (future :: ready (self)) . boxed () } }
};
}
