// Generated macro for impl_182 (impl)
macro_rules! Depcrate_tokens_token_streamimpl_182 {
() => {
// Module: crate::tokens::token_stream
// Provides: {"impl_182"}
// Dependencies: {}
impl Delimiter { # [doc = " The opening delimiter"] pub fn open (self) -> char { match self { Self :: Bracket => '[' , Self :: Brace => '{' , Self :: Parenthesis => '(' , } } # [doc = " The closing delimiter"] pub fn close (self) -> char { match self { Self :: Bracket => ']' , Self :: Brace => '}' , Self :: Parenthesis => ')' , } } }
};
}
