// Generated macro for Schema (trait)
macro_rules! Depcrate_schemaSchema {
() => {
// Module: crate::schema
// Provides: {"Schema"}
// Dependencies: {}
# [doc = " Schema defines the requirements for schemas that can be used for operations. Typically this is"] # [doc = " just an `Arc<RootNode<...>>` and you should not have to implement it yourself."] pub trait Schema : Unpin + Clone + Send + Sync + 'static { # [doc = " The context type."] type Context : Unpin + Send + Sync ; # [doc = " The scalar value type."] type ScalarValue : ScalarValue + Send + Sync ; # [doc = " The query type info."] type QueryTypeInfo : Send + Sync ; # [doc = " The query type."] type Query : GraphQLTypeAsync < Self :: ScalarValue , Context = Self :: Context , TypeInfo = Self :: QueryTypeInfo > + Send ; # [doc = " The mutation type info."] type MutationTypeInfo : Send + Sync ; # [doc = " The mutation type."] type Mutation : GraphQLTypeAsync < Self :: ScalarValue , Context = Self :: Context , TypeInfo = Self :: MutationTypeInfo , > + Send ; # [doc = " The subscription type info."] type SubscriptionTypeInfo : Send + Sync ; # [doc = " The subscription type."] type Subscription : GraphQLSubscriptionType < Self :: ScalarValue , Context = Self :: Context , TypeInfo = Self :: SubscriptionTypeInfo , > + Send ; # [doc = " Returns the root node for the schema."] fn root_node (& self ,) -> & RootNode < Self :: Query , Self :: Mutation , Self :: Subscription , Self :: ScalarValue > ; }
};
}
