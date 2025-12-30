// Generated macro for generics_to_extra_generics (function)
macro_rules! Depcrategenerics_to_extra_generics {
() => {
// Module: crate
// Provides: {"generics_to_extra_generics"}
// Dependencies: {}
# [doc = " If the input is non-empty remove the enclosing `<` and `>` and prepend a comma."] # [doc = ""] # [doc = " Does not check whether the enclosing tokens actually are `<` and `>`."] fn generics_to_extra_generics (generics : & impl ToTokens) -> proc_macro2 :: TokenStream { let mut generics_tokens = proc_macro2 :: TokenStream :: new () ; generics . to_tokens (& mut generics_tokens) ; let mut generics_tokens = generics_tokens . into_iter () . collect :: < Vec < _ > > () ; if ! generics_tokens . is_empty () { generics_tokens [0] = quote ! (,) . into_iter () . next () . unwrap () ; generics_tokens . pop () ; } let mut extra_tokens = proc_macro2 :: TokenStream :: new () ; extra_tokens . extend (generics_tokens) ; extra_tokens }
};
}
