// Generated macro for impl_232 (impl)
macro_rules! Depcrate_graphql_interfaceimpl_232 {
() => {
// Module: crate::graphql_interface
// Provides: {"impl_232"}
// Dependencies: {}
impl ToTokens for Definition { fn to_tokens (& self , into : & mut TokenStream) { self . generate_enum_tokens () . to_tokens (into) ; self . impl_graphql_interface_tokens () . to_tokens (into) ; self . impl_output_type_tokens () . to_tokens (into) ; self . impl_graphql_type_tokens () . to_tokens (into) ; self . impl_graphql_value_tokens () . to_tokens (into) ; self . impl_graphql_value_async_tokens () . to_tokens (into) ; self . impl_reflection_traits_tokens () . to_tokens (into) ; self . impl_field_meta_tokens () . to_tokens (into) ; self . impl_field_tokens () . to_tokens (into) ; self . impl_async_field_tokens () . to_tokens (into) ; } }
};
}
