// Generated macro for impl_39 (impl)
macro_rules! Depcrate_subscriptionimpl_39 {
() => {
// Module: crate::subscription
// Provides: {"impl_39"}
// Dependencies: {}
impl < S , E > GraphQLWebSocket < SplitSink < S , Message > , SplitStream < S > , E , DefaultOnConnInitType , DefaultOnPingType , > where S : Stream < Item = Result < Message , IoError > > + Sink < Message > , E : Executor , { # [doc = " Create a [`GraphQLWebSocket`] object."] pub fn new (stream : S , executor : E , protocol : GraphQLProtocol) -> Self { let (sink , stream) = stream . split () ; GraphQLWebSocket :: new_with_pair (sink , stream , executor , protocol) } }
};
}
