// Generated macro for impl_40 (impl)
macro_rules! Depcrate_subscriptionimpl_40 {
() => {
// Module: crate::subscription
// Provides: {"impl_40"}
// Dependencies: {}
impl < Sink , Stream , E > GraphQLWebSocket < Sink , Stream , E , DefaultOnConnInitType , DefaultOnPingType > where Sink : futures_util :: sink :: Sink < Message > , Stream : futures_util :: stream :: Stream < Item = Result < Message , IoError > > , E : Executor , { # [doc = " Create a [`GraphQLWebSocket`] object with sink and stream objects."] pub fn new_with_pair (sink : Sink , stream : Stream , executor : E , protocol : GraphQLProtocol ,) -> Self { GraphQLWebSocket { sink , stream , executor , data : Data :: default () , on_connection_init : default_on_connection_init , on_ping : default_on_ping , protocol , keepalive_timeout : None , } } }
};
}
