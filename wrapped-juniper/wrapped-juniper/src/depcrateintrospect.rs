// Generated macro for introspect (function)
macro_rules! Depcrateintrospect {
() => {
// Module: crate
// Provides: {"introspect"}
// Dependencies: {}
# [doc = " Executes the [canonical introspection query][0] in the provided schema."] # [doc = ""] # [doc = " [0]: https://github.com/graphql/graphql-js/blob/v16.11.0/src/utilities/getIntrospectionQuery.ts#L75"] pub fn introspect < S , QueryT , MutationT , SubscriptionT > (root_node : & RootNode < QueryT , MutationT , SubscriptionT , S > , context : & QueryT :: Context , format : IntrospectionFormat ,) -> Result < (Value < S > , Vec < ExecutionError < S > >) , GraphQLError > where S : ScalarValue , QueryT : GraphQLType < S > , MutationT : GraphQLType < S , Context = QueryT :: Context > , SubscriptionT : GraphQLType < S , Context = QueryT :: Context > , { execute_sync (match format { IntrospectionFormat :: All => INTROSPECTION_QUERY , IntrospectionFormat :: WithoutDescriptions => INTROSPECTION_QUERY_WITHOUT_DESCRIPTIONS , } , None , root_node , & Variables :: new () , context ,) }
};
}
