// Generated macro for RootNode (struct)
macro_rules! Depcrate_schema_modelRootNode {
() => {
// Module: crate::schema::model
// Provides: {"RootNode"}
// Dependencies: {}
# [doc = " Root query node of a schema"] # [doc = ""] # [doc = " This brings the mutation, subscription and query types together,"] # [doc = " and provides the predefined metadata fields."] # [derive (Debug)] pub struct RootNode < QueryT : GraphQLType < S > , MutationT : GraphQLType < S > , SubscriptionT : GraphQLType < S > , S = DefaultScalarValue , > where S : ScalarValue , { # [doc (hidden)] pub query_type : QueryT , # [doc (hidden)] pub query_info : QueryT :: TypeInfo , # [doc (hidden)] pub mutation_type : MutationT , # [doc (hidden)] pub mutation_info : MutationT :: TypeInfo , # [doc (hidden)] pub subscription_type : SubscriptionT , # [doc (hidden)] pub subscription_info : SubscriptionT :: TypeInfo , # [doc (hidden)] pub schema : SchemaType < S > , # [doc (hidden)] pub introspection_disabled : bool , }
};
}
