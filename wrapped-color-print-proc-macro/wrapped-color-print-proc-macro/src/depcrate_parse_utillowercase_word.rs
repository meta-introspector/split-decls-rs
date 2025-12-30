// Generated macro for lowercase_word (function)
macro_rules! Depcrate_parse_utillowercase_word {
() => {
// Module: crate::parse::util
// Provides: {"lowercase_word"}
// Dependencies: {}
# [doc = " Parses a lowercase word."] pub fn lowercase_word (input : Input < '_ >) -> Result < '_ , & str > { let (input , word) = alpha1 (input) ? ; if word . chars () . all (| c | c . is_ascii_lowercase ()) { Ok ((input , word)) } else { Err (Err :: Error (Error :: new (input , ErrorKind :: Alpha , None))) } }
};
}
