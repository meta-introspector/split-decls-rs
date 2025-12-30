// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a , QueryT , MutationT , SubscriptionT , CtxT , S > SubscriptionCoordinator < 'a , CtxT , S > for Coordinator < QueryT , MutationT , SubscriptionT , CtxT , S > where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Sync , S : ScalarValue + Send + Sync + 'a , { type Connection = Connection < 'a , S > ; type Error = GraphQLError ; fn subscribe (& 'a self , req : & 'a GraphQLRequest < S > , context : & 'a CtxT ,) -> BoxFuture < 'a , Result < Self :: Connection , Self :: Error > > { juniper :: http :: resolve_into_stream (req , & self . root_node , context) . map_ok (| (stream , errors) | Connection :: from_stream (stream , errors)) . boxed () } }
};
}
