// Generated macro for impl_436 (impl)
macro_rules! Depcrate_schema_modelimpl_436 {
() => {
// Module: crate::schema::model
// Provides: {"impl_436"}
// Dependencies: {}
impl < QueryT , MutationT , SubscriptionT > RootNode < QueryT , MutationT , SubscriptionT , DefaultScalarValue > where QueryT : GraphQLType < DefaultScalarValue , TypeInfo = () > , MutationT : GraphQLType < DefaultScalarValue , TypeInfo = () > , SubscriptionT : GraphQLType < DefaultScalarValue , TypeInfo = () > , { # [doc = " Constructs a new [`RootNode`] from `query`, `mutation` and `subscription` nodes,"] # [doc = " parametrizing it with a [`DefaultScalarValue`]."] pub fn new (query : QueryT , mutation : MutationT , subscription : SubscriptionT) -> Self { Self :: new_with_info (query , mutation , subscription , () , () , ()) } }
};
}
