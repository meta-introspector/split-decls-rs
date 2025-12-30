// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl < QueryT , MutationT , SubscriptionT , CtxT , S > Coordinator < QueryT , MutationT , SubscriptionT , CtxT , S > where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { # [doc = " Builds new [`Coordinator`] with specified `root_node`"] pub fn new (root_node : juniper :: RootNode < QueryT , MutationT , SubscriptionT , S >) -> Self { Self { root_node } } }
};
}
