// Generated macro for impl_78 (impl)
macro_rules! Depcrate_schemaimpl_78 {
() => {
// Module: crate::schema
// Provides: {"impl_78"}
// Dependencies: {}
impl < QueryT , MutationT , SubscriptionT , CtxT , S > Clone for ArcSchema < QueryT , MutationT , SubscriptionT , CtxT , S > where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send + 'static , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Unpin + Send + Sync , S : ScalarValue + Send + Sync + 'static , { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
};
}
