// Generated macro for respan (function)
macro_rules! Depcrate_codegenrespan {
() => {
// Module: crate::codegen
// Provides: {"respan"}
// Dependencies: {}
# [doc = " Converts `span` into a stream of tokens, and attempts to ensure that `input`"] # [doc = " has all the appropriate span information so errors in it point to `span`."] fn respan (input : TokenStream , span : & dyn ToTokens) -> TokenStream { let mut first_span = Span :: call_site () ; let mut last_span = Span :: call_site () ; let mut spans = TokenStream :: new () ; span . to_tokens (& mut spans) ; for (i , token) in spans . into_iter () . enumerate () { if i == 0 { first_span = Span :: call_site () . located_at (token . span ()) ; } last_span = Span :: call_site () . located_at (token . span ()) ; } let mut new_tokens = Vec :: new () ; for (i , mut token) in input . into_iter () . enumerate () { if i == 0 { token . set_span (first_span) ; } else { token . set_span (last_span) ; } new_tokens . push (token) ; } new_tokens . into_iter () . collect () }
};
}
