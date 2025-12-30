// Generated macro for word (function)
macro_rules! Depcrate_parse_utilword {
() => {
// Module: crate::parse::util
// Provides: {"word"}
// Dependencies: {}
# [doc = " Parses a word made only by alpha characters ('a' => 'z' and 'A' => 'Z'), and checks if this"] # [doc = " word matches exactly the given parser."] pub fn word < 'a , P > (mut word_parser : P) -> impl Parser < 'a , & 'a str > where P : Parser < 'a , & 'a str > , { move | input | { let (input , word) = alpha1 (input) ? ; match word_parser (word) { Ok ((_ , parsed_word)) => { if word == parsed_word { Ok ((input , word)) } else { Err (Err :: Error (Error :: new (input , ErrorKind :: Alpha , None))) } } Err (e) => Err (e) , } } }
};
}
