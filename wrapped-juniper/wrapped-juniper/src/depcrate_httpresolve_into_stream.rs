// Generated macro for resolve_into_stream (function)
macro_rules! Depcrate_httpresolve_into_stream {
() => {
// Module: crate::http
// Provides: {"resolve_into_stream"}
// Dependencies: {}
# [doc = " Resolve a GraphQL subscription into `Value<ValuesStream<S>` using the"] # [doc = " specified schema and context."] # [doc = " This is a wrapper around the `resolve_into_stream` function exposed at the top"] # [doc = " level of this crate."] pub async fn resolve_into_stream < 'req , 'rn , 'ctx , 'a , QueryT , MutationT , SubscriptionT , S > (req : & 'req GraphQLRequest < S > , root_node : & 'rn RootNode < QueryT , MutationT , SubscriptionT , S > , context : & 'ctx QueryT :: Context ,) -> Result < (Value < ValuesStream < 'a , S > > , Vec < ExecutionError < S > >) , GraphQLError > where 'req : 'a , 'rn : 'a , 'ctx : 'a , QueryT : GraphQLTypeAsync < S > , QueryT :: TypeInfo : Sync , QueryT :: Context : Sync , MutationT : GraphQLTypeAsync < S , Context = QueryT :: Context > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = QueryT :: Context > , SubscriptionT :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { let op = req . operation_name . as_deref () ; let vars = req . variables () ; crate :: resolve_into_stream (& req . query , op , root_node , & vars , context) . await }
};
}
