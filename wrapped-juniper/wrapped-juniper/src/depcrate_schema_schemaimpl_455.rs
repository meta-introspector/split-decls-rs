// Generated macro for impl_455 (impl)
macro_rules! Depcrate_schema_schemaimpl_455 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_455"}
// Dependencies: {}
impl < S , QueryT , MutationT , SubscriptionT > GraphQLType < S > for RootNode < QueryT , MutationT , SubscriptionT , S > where S : ScalarValue , QueryT : GraphQLType < S > , MutationT : GraphQLType < S , Context = QueryT :: Context > , SubscriptionT : GraphQLType < S , Context = QueryT :: Context > , { fn name (info : & Self :: TypeInfo) -> Option < ArcStr > { QueryT :: name (info) } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { QueryT :: meta (info , registry) } }
};
}
