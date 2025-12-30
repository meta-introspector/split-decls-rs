// Generated macro for impl_60 (impl)
macro_rules! Depcrate_graphql_wsimpl_60 {
() => {
// Module: crate::graphql_ws
// Provides: {"impl_60"}
// Dependencies: {}
impl < S : Schema > Reaction < S > { # [doc = " Converts the reaction into a one-item stream."] fn into_stream (self) -> BoxStream < 'static , Self > { stream :: once (future :: ready (self)) . boxed () } }
};
}
