// Generated macro for GraphQLSubscriptionType (trait)
macro_rules! Depcrate_types_subscriptionsGraphQLSubscriptionType {
() => {
// Module: crate::types::subscriptions
// Provides: {"GraphQLSubscriptionType"}
// Dependencies: {}
# [doc = " Extension of [`GraphQLType`] trait with asynchronous [subscription][1] execution logic."] # [doc = ""] # [doc = " It's automatically implemented for [`GraphQLSubscriptionValue`] and [`GraphQLType`]"] # [doc = " implementers, so doesn't require manual or code-generated implementation."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Subscription"] pub trait GraphQLSubscriptionType < S = DefaultScalarValue > : GraphQLSubscriptionValue < S > + GraphQLType < S > where Self :: Context : Sync , Self :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { }
};
}
