// Generated macro for token_set_works_for_tokens (function)
macro_rules! Depcrate_token_settoken_set_works_for_tokens {
() => {
// Module: crate::token_set
// Provides: {"token_set_works_for_tokens"}
// Dependencies: {}
# [test] fn token_set_works_for_tokens () { use crate :: SyntaxKind :: * ; let ts = TokenSet :: new (& [EOF , SHEBANG]) ; assert ! (ts . contains (EOF)) ; assert ! (ts . contains (SHEBANG)) ; assert ! (! ts . contains (PLUS)) ; }
};
}
