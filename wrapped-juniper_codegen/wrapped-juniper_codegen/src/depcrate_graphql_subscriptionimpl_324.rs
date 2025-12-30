// Generated macro for impl_324 (impl)
macro_rules! Depcrate_graphql_subscriptionimpl_324 {
() => {
// Module: crate::graphql_subscription
// Provides: {"impl_324"}
// Dependencies: {}
impl ToTokens for Definition < Subscription > { fn to_tokens (& self , into : & mut TokenStream) { self . impl_output_type_tokens () . to_tokens (into) ; self . impl_graphql_type_tokens () . to_tokens (into) ; self . impl_graphql_value_tokens () . to_tokens (into) ; self . impl_graphql_subscription_value_tokens () . to_tokens (into) ; } }
};
}
