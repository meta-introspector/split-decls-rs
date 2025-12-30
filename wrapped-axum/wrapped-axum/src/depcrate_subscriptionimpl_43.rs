// Generated macro for impl_43 (impl)
macro_rules! Depcrate_subscriptionimpl_43 {
() => {
// Module: crate::subscription
// Provides: {"impl_43"}
// Dependencies: {}
impl < S , E > GraphQLWebSocket < SplitSink < S , Message > , SplitStream < S > , E , DefaultOnConnInitType , DefaultOnPingType , > where S : Stream < Item = Result < Message , Error > > + Sink < Message > , E : Executor , { # [doc = " Create a [`GraphQLWebSocket`] object."] pub fn new (stream : S , executor : E , protocol : GraphQLProtocol) -> Self { let (sink , stream) = stream . split () ; GraphQLWebSocket :: new_with_pair (sink , stream , executor , protocol) } }
};
}
