// Generated macro for has_derive_arbitrary_attr (function)
macro_rules! Depcrate_arbitrary_in_macro_ruleshas_derive_arbitrary_attr {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"has_derive_arbitrary_attr"}
// Dependencies: {}
fn has_derive_arbitrary_attr (attrs : & [syn :: Attribute]) -> bool { for attr in attrs { if attr . path () . is_ident ("derive") { if let syn :: Meta :: List (meta_list) = & attr . meta { let tokens_str = meta_list . tokens . to_string () ; if tokens_str . contains ("Arbitrary") { return true ; } } } } false }
};
}
