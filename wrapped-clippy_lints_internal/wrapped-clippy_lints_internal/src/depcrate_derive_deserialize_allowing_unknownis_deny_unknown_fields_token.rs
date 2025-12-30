// Generated macro for is_deny_unknown_fields_token (function)
macro_rules! Depcrate_derive_deserialize_allowing_unknownis_deny_unknown_fields_token {
() => {
// Module: crate::derive_deserialize_allowing_unknown
// Provides: {"is_deny_unknown_fields_token"}
// Dependencies: {}
fn is_deny_unknown_fields_token (tt : & TokenTree) -> bool { if let TokenTree :: Token (token , _) = tt && token . ident () . is_some_and (| (token , _) | token . as_str () == "deny_unknown_fields") { true } else { false } }
};
}
