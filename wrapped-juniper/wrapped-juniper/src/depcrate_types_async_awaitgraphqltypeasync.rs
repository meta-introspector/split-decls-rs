// Generated macro for GraphQLTypeAsync (trait)
macro_rules! Depcrate_types_async_awaitGraphQLTypeAsync {
() => {
// Module: crate::types::async_await
// Provides: {"GraphQLTypeAsync"}
// Dependencies: {}
# [doc = " Extension of [`GraphQLType`] trait with asynchronous queries/mutations resolvers."] # [doc = ""] # [doc = " It's automatically implemented for [`GraphQLValueAsync`] and [`GraphQLType`] implementers, so"] # [doc = " doesn't require manual or code-generated implementation."] pub trait GraphQLTypeAsync < S = DefaultScalarValue > : GraphQLValueAsync < S > + GraphQLType < S > where Self :: Context : Sync , Self :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { }
};
}
