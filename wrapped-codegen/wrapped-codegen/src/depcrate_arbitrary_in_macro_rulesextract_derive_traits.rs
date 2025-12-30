// Generated macro for extract_derive_traits (function)
macro_rules! Depcrate_arbitrary_in_macro_rulesextract_derive_traits {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"extract_derive_traits"}
// Dependencies: {}
fn extract_derive_traits (attrs : & [syn :: Attribute]) -> Vec < String > { for attr in attrs { if attr . path () . is_ident ("derive") { if let syn :: Meta :: List (meta_list) = & attr . meta { let tokens_str = meta_list . tokens . to_string () ; return tokens_str . split (',') . map (| s | s . trim () . to_string ()) . collect () ; } } } vec ! [] }
};
}
