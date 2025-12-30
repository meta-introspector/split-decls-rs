// Generated macro for impl_687 (impl)
macro_rules! Depcrate_dynamic_subscriptionimpl_687 {
() => {
// Module: crate::dynamic::subscription
// Provides: {"impl_687"}
// Dependencies: {}
impl < 'a > SubscriptionFieldFuture < 'a > { # [doc = " Create a ResolverFuture"] pub fn new < Fut , S , T > (future : Fut) -> Self where Fut : Future < Output = Result < S > > + Send + 'a , S : Stream < Item = Result < T > > + Send + 'a , T : Into < FieldValue < 'a > > + Send + 'a , { Self (async move { let res = future . await ? . map_ok (Into :: into) ; Ok (res . boxed ()) } . boxed () ,) } }
};
}
