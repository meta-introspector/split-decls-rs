// Generated macro for impl_195 (impl)
macro_rules! Depcrate_graphql_input_objectimpl_195 {
() => {
// Module: crate::graphql_input_object
// Provides: {"impl_195"}
// Dependencies: {}
impl ToTokens for Definition { fn to_tokens (& self , into : & mut TokenStream) { self . impl_input_type_tokens () . to_tokens (into) ; self . impl_graphql_type_tokens () . to_tokens (into) ; self . impl_graphql_value_tokens () . to_tokens (into) ; self . impl_graphql_value_async_tokens () . to_tokens (into) ; self . impl_from_input_value_tokens () . to_tokens (into) ; self . impl_to_input_value_tokens () . to_tokens (into) ; self . impl_reflection_traits_tokens () . to_tokens (into) ; self . impl_field_meta_tokens () . to_tokens (into) ; } }
};
}
