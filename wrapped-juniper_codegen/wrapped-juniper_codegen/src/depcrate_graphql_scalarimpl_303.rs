// Generated macro for impl_303 (impl)
macro_rules! Depcrate_graphql_scalarimpl_303 {
() => {
// Module: crate::graphql_scalar
// Provides: {"impl_303"}
// Dependencies: {}
impl ToTokens for Definition { fn to_tokens (& self , into : & mut TokenStream) { self . impl_output_and_input_type_tokens () . to_tokens (into) ; self . impl_type_tokens () . to_tokens (into) ; self . impl_value_tokens () . to_tokens (into) ; self . impl_value_async_tokens () . to_tokens (into) ; self . impl_to_scalar_value_tokens () . to_tokens (into) ; self . impl_to_input_value_tokens () . to_tokens (into) ; self . impl_from_scalar_value_tokens () . to_tokens (into) ; self . impl_from_input_value_tokens () . to_tokens (into) ; self . impl_parse_scalar_value_tokens () . to_tokens (into) ; self . impl_reflection_traits_tokens () . to_tokens (into) ; } }
};
}
