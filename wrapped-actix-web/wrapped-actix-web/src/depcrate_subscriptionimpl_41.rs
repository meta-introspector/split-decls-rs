// Generated macro for impl_41 (impl)
macro_rules! Depcrate_subscriptionimpl_41 {
() => {
// Module: crate::subscription
// Provides: {"impl_41"}
// Dependencies: {}
impl < E > GraphQLSubscription < E , DefaultOnConnInitType , DefaultOnPingType > { # [doc = " Create a GraphQL subscription builder."] pub fn new (executor : E) -> Self { Self { executor , data : Default :: default () , on_connection_init : default_on_connection_init , on_ping : default_on_ping , keepalive_timeout : None , } } }
};
}
