// Generated macro for pick_best_token (function)
macro_rules! Depcrate_helperspick_best_token {
() => {
// Module: crate::helpers
// Provides: {"pick_best_token"}
// Dependencies: {}
# [doc = " Picks the token with the highest rank returned by the passed in function."] pub fn pick_best_token (tokens : TokenAtOffset < SyntaxToken > , f : impl Fn (SyntaxKind) -> usize ,) -> Option < SyntaxToken > { tokens . max_by_key (move | t | f (t . kind ())) }
};
}
