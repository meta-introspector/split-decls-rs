// Generated macro for TokenSource (trait)
macro_rules! Depcrate_internTokenSource {
() => {
// Module: crate::intern
// Provides: {"TokenSource"}
// Dependencies: {}
pub trait TokenSource { type Token : Hash + Eq ; type Tokenizer : Iterator < Item = Self :: Token > ; fn tokenize (& self) -> Self :: Tokenizer ; fn estimate_tokens (& self) -> u32 ; }
};
}
