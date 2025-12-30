// Generated macro for SubscriptionField (struct)
macro_rules! Depcrate_dynamic_subscriptionSubscriptionField {
() => {
// Module: crate::dynamic::subscription
// Provides: {"SubscriptionField"}
// Dependencies: {}
# [doc = " A GraphQL subscription field"] pub struct SubscriptionField { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) arguments : IndexMap < String , InputValue > , pub (crate) ty : TypeRef , pub (crate) resolver_fn : BoxResolverFn , pub (crate) deprecation : Deprecation , }
};
}
