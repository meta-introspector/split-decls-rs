// Generated macro for pick_token (function)
macro_rules! Depcrate_helperspick_token {
() => {
// Module: crate::helpers
// Provides: {"pick_token"}
// Dependencies: {}
pub fn pick_token < T : AstToken > (mut tokens : TokenAtOffset < SyntaxToken >) -> Option < T > { tokens . find_map (T :: cast) }
};
}
