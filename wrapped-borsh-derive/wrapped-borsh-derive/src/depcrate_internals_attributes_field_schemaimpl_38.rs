// Generated macro for impl_38 (impl)
macro_rules! Depcrate_internals_attributes_field_schemaimpl_38 {
() => {
// Module: crate::internals::attributes::field::schema
// Provides: {"impl_38"}
// Dependencies: {}
impl ToTokens for ParameterOverride { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { self . order_param . to_tokens (tokens) ; self . arrow_token . to_tokens (tokens) ; self . override_type . to_tokens (tokens) ; } }
};
}
