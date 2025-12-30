// Generated macro for CheckTyContext (struct)
macro_rules! Depcrate_typesCheckTyContext {
() => {
// Module: crate::types
// Provides: {"CheckTyContext"}
// Dependencies: {}
# [expect (clippy :: struct_excessive_bools)] # [derive (Clone , Copy , Default)] struct CheckTyContext { is_in_trait_impl : bool , # [doc = " `true` for types on local variables and in closure signatures."] in_body : bool , # [doc = " `true` for types that are part of the public API."] is_exported : bool , is_nested_call : bool , }
};
}
