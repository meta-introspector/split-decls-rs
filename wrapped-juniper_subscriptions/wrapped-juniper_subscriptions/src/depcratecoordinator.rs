// Generated macro for Coordinator (struct)
macro_rules! DepcrateCoordinator {
() => {
// Module: crate
// Provides: {"Coordinator"}
// Dependencies: {}
# [doc = " Simple [`SubscriptionCoordinator`] implementation:"] # [doc = " - contains the schema"] # [doc = " - handles subscription start"] pub struct Coordinator < QueryT , MutationT , SubscriptionT , CtxT , S > where QueryT : GraphQLTypeAsync < S , Context = CtxT > + Send , QueryT :: TypeInfo : Send + Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > + Send , MutationT :: TypeInfo : Send + Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > + Send , SubscriptionT :: TypeInfo : Send + Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { root_node : juniper :: RootNode < QueryT , MutationT , SubscriptionT , S > , }
};
}
