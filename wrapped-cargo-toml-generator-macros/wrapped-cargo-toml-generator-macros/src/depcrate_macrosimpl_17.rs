// Generated macro for impl_17 (impl)
macro_rules! Depcrate_macrosimpl_17 {
() => {
// Module: crate::macros
// Provides: {"impl_17"}
// Dependencies: {}
impl ToTokens for WrappedCargoToml { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let serialized = serde_json :: to_string (& self . 0) . expect ("Failed to serialize CargoToml to JSON") ; tokens . extend (quote ! { { let cargo_toml_json = # serialized ; serde_json :: from_str (& cargo_toml_json) . expect ("Failed to deserialize CargoToml from JSON") } }) ; } }
};
}
