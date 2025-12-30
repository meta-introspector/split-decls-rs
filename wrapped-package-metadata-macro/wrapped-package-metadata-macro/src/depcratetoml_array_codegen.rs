// Generated macro for toml_array_codegen (function)
macro_rules! Depcratetoml_array_codegen {
() => {
// Module: crate
// Provides: {"toml_array_codegen"}
// Dependencies: {}
fn toml_array_codegen (array : & Array) -> proc_macro2 :: TokenStream { let statements = array . iter () . flat_map (| val | { let val = toml_value_codegen (val) ; quote ! { # val , } }) . collect :: < proc_macro2 :: TokenStream > () ; quote ! { { [# statements] } } }
};
}
