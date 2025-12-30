// Generated macro for is_present (function)
macro_rules! Depcrate_parse_utilis_present {
() => {
// Module: crate::parse::util
// Provides: {"is_present"}
// Dependencies: {}
# [doc = " Creates a parser which makes the parser optional and returns true if the parse was successful."] pub fn is_present < 'a , P , V > (parser : P) -> impl Parser < 'a , bool > where P : Parser < 'a , V > , { map (opt (parser) , | v | v . is_some ()) }
};
}
