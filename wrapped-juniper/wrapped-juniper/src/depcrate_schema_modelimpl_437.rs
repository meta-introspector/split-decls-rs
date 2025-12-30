// Generated macro for impl_437 (impl)
macro_rules! Depcrate_schema_modelimpl_437 {
() => {
// Module: crate::schema::model
// Provides: {"impl_437"}
// Dependencies: {}
impl < QueryT , MutationT , SubscriptionT , S > RootNode < QueryT , MutationT , SubscriptionT , S > where S : ScalarValue , QueryT : GraphQLType < S , TypeInfo = () > , MutationT : GraphQLType < S , TypeInfo = () > , SubscriptionT : GraphQLType < S , TypeInfo = () > , { # [doc = " Constructs a new [`RootNode`] from `query`, `mutation` and `subscription` nodes,"] # [doc = " parametrizing it with the provided [`ScalarValue`]."] pub fn new_with_scalar_value (query : QueryT , mutation : MutationT , subscription : SubscriptionT ,) -> Self { RootNode :: new_with_info (query , mutation , subscription , () , () , ()) } }
};
}
