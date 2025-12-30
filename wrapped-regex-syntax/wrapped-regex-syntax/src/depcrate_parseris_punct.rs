// Generated macro for is_punct (function)
macro_rules! Depcrate_parseris_punct {
() => {
// Module: crate::parser
// Provides: {"is_punct"}
// Dependencies: {}
# [doc = " Returns true if the give character has significance in a regex."] pub fn is_punct (c : char) -> bool { match c { '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '#' => true , _ => false , } }
};
}
