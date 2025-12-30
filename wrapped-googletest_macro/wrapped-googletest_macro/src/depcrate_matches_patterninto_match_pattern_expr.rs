// Generated macro for into_match_pattern_expr (function)
macro_rules! Depcrate_matches_patterninto_match_pattern_expr {
() => {
// Module: crate::matches_pattern
// Provides: {"into_match_pattern_expr"}
// Dependencies: {}
# [doc = " Returns a pattern match expression as long as `stream` is a valid pattern."] # [doc = " Otherwise, returns failure."] fn into_match_pattern_expr (stream : TokenStream) -> syn :: Result < TokenStream > { Pat :: parse_multi . parse2 (stream . clone ()) ? ; Ok (quote ! { googletest :: matchers :: __internal_unstable_do_not_depend_on_these :: pattern_only (| v | matches ! (v , # stream) , concat ! ("is " , stringify ! (# stream)) , concat ! ("is not " , stringify ! (# stream))) }) }
};
}
