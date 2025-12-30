// Generated macro for BoxResolverFn (type)
macro_rules! Depcrate_dynamic_subscriptionBoxResolverFn {
() => {
// Module: crate::dynamic::subscription
// Provides: {"BoxResolverFn"}
// Dependencies: {}
type BoxResolverFn = Arc < (dyn for < 'a > Fn (ResolverContext < 'a >) -> SubscriptionFieldFuture < 'a > + Send + Sync) > ;
};
}
