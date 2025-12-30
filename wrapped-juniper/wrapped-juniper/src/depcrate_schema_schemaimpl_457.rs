// Generated macro for impl_457 (impl)
macro_rules! Depcrate_schema_schemaimpl_457 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_457"}
// Dependencies: {}
impl < S , QueryT , MutationT , SubscriptionT > GraphQLValueAsync < S > for RootNode < QueryT , MutationT , SubscriptionT , S > where QueryT : GraphQLTypeAsync < S > , QueryT :: TypeInfo : Sync , QueryT :: Context : Sync , MutationT : GraphQLTypeAsync < S , Context = QueryT :: Context > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLType < S , Context = QueryT :: Context > + Sync , SubscriptionT :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { fn resolve_field_async < 'b > (& 'b self , info : & 'b Self :: TypeInfo , field_name : & 'b str , arguments : & 'b Arguments < S > , executor : & 'b Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'b , ExecutionResult < S > > { use std :: future ; match field_name { "__schema" | "__type" => { let v = self . resolve_field (info , field_name , arguments , executor) ; Box :: pin (future :: ready (v)) } _ => self . query_type . resolve_field_async (info , field_name , arguments , executor) , } } }
};
}
