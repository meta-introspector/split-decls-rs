// Generated macro for impl_167 (impl)
macro_rules! Depcrate_graphql_enumimpl_167 {
() => {
// Module: crate::graphql_enum
// Provides: {"impl_167"}
// Dependencies: {}
impl ToTokens for Definition { fn to_tokens (& self , into : & mut TokenStream) { self . impl_input_and_output_type_tokens () . to_tokens (into) ; self . impl_graphql_type_tokens () . to_tokens (into) ; self . impl_graphql_value_tokens () . to_tokens (into) ; self . impl_graphql_value_async_tokens () . to_tokens (into) ; self . impl_from_input_value_tokens () . to_tokens (into) ; self . impl_to_input_value_tokens () . to_tokens (into) ; self . impl_reflection_traits_tokens () . to_tokens (into) ; } }
};
}
