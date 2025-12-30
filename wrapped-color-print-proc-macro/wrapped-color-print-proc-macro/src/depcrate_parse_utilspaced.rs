// Generated macro for spaced (function)
macro_rules! Depcrate_parse_utilspaced {
() => {
// Module: crate::parse::util
// Provides: {"spaced"}
// Dependencies: {}
# [doc = " Creates a parser which accpets spaces around the original parsed input."] pub fn spaced < 'a , P , V > (parser : P) -> impl Parser < 'a , V > where P : Parser < 'a , V > , { delimited (multispace0 , parser , multispace0 ,) }
};
}
