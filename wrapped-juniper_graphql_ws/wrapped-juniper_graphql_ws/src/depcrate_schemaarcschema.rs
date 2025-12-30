// Generated macro for ArcSchema (struct)
macro_rules! Depcrate_schemaArcSchema {
() => {
// Module: crate::schema
// Provides: {"ArcSchema"}
// Dependencies: {}
# [doc = " This exists as a work-around for this issue: https://github.com/rust-lang/rust/issues/64552"] # [doc = ""] # [doc = " It can be used in generators where using Arc directly would result in an error."] # [doc (hidden)] pub struct ArcSchema < QueryT , MutationT , SubscriptionT , CtxT , S > (pub Arc < RootNode < QueryT , MutationT , SubscriptionT , S > > ,) where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send + 'static , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Unpin + Send + Sync , S : ScalarValue + Send + Sync + 'static ;
};
}
