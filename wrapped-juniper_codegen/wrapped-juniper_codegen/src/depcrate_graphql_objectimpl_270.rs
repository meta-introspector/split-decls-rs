// Generated macro for impl_270 (impl)
macro_rules! Depcrate_graphql_objectimpl_270 {
() => {
// Module: crate::graphql_object
// Provides: {"impl_270"}
// Dependencies: {}
impl ToTokens for Definition < Query > { fn to_tokens (& self , into : & mut TokenStream) { self . impl_graphql_object_tokens () . to_tokens (into) ; self . impl_output_type_tokens () . to_tokens (into) ; self . impl_graphql_type_tokens () . to_tokens (into) ; self . impl_graphql_value_tokens () . to_tokens (into) ; self . impl_graphql_value_async_tokens () . to_tokens (into) ; self . impl_reflection_traits_tokens () . to_tokens (into) ; self . impl_field_meta_tokens () . to_tokens (into) ; self . impl_field_tokens () . to_tokens (into) ; self . impl_async_field_tokens () . to_tokens (into) ; } }
};
}
