// Generated macro for ExtractTypeFromStream (trait)
macro_rules! Depcrate_macros_helper_subscriptionExtractTypeFromStream {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"ExtractTypeFromStream"}
// Dependencies: {}
# [doc = " This trait is used in `juniper::graphql_subscription` macro to get stream's"] # [doc = " item type that implements `GraphQLValue` from type alias provided"] # [doc = " by user."] pub trait ExtractTypeFromStream < T , S > where S : ScalarValue , { # [doc = " Stream's return Value that will be returned if"] # [doc = " no errors occured. Is used to determine field type in"] # [doc = " `#[juniper::graphql_subscription]`"] type Item : GraphQLValue < S > ; }
};
}
