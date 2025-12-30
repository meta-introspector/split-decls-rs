// Generated macro for Input (struct)
macro_rules! Depcrate_inputInput {
() => {
// Module: crate::input
// Provides: {"Input"}
// Dependencies: {}
# [doc = " Input for the parser -- a sequence of tokens."] # [doc = ""] # [doc = " As of now, parser doesn't have access to the *text* of the tokens, and makes"] # [doc = " decisions based solely on their classification. Unlike `LexerToken`, the"] # [doc = " `Tokens` doesn't include whitespace and comments. Main input to the parser."] # [doc = ""] # [doc = " Struct of arrays internally, but this shouldn't really matter."] pub struct Input { kind : Vec < SyntaxKind > , joint : Vec < bits > , contextual_kind : Vec < SyntaxKind > , }
};
}
