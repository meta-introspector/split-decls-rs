// Generated macro for impl_80 (impl)
macro_rules! Depcrate_schemaimpl_80 {
() => {
// Module: crate::schema
// Provides: {"impl_80"}
// Dependencies: {}
impl < QueryT , MutationT , SubscriptionT , CtxT , S > Schema for Arc < RootNode < QueryT , MutationT , SubscriptionT , S > > where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send + 'static , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Unpin + Send + Sync , S : ScalarValue + Send + Sync + 'static , { type Context = CtxT ; type ScalarValue = S ; type QueryTypeInfo = QueryT :: TypeInfo ; type Query = QueryT ; type MutationTypeInfo = MutationT :: TypeInfo ; type Mutation = MutationT ; type SubscriptionTypeInfo = SubscriptionT :: TypeInfo ; type Subscription = SubscriptionT ; fn root_node (& self) -> & RootNode < QueryT , MutationT , SubscriptionT , S > { self } }
};
}
