// Generated macro for uppercase_word (function)
macro_rules! Depcrate_parse_utiluppercase_word {
() => {
// Module: crate::parse::util
// Provides: {"uppercase_word"}
// Dependencies: {}
# [doc = " Parses an uppercase word."] pub fn uppercase_word (input : Input < '_ >) -> Result < '_ , & str > { let (input , word) = alpha1 (input) ? ; if word . chars () . all (| c | c . is_ascii_uppercase ()) { Ok ((input , word)) } else { Err (Err :: Error (Error :: new (input , ErrorKind :: Alpha , None))) } }
};
}
