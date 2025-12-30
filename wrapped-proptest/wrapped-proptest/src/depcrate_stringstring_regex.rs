// Generated macro for string_regex (function)
macro_rules! Depcrate_stringstring_regex {
() => {
// Module: crate::string
// Provides: {"string_regex"}
// Dependencies: {}
# [doc = " Creates a strategy which generates strings matching the given regular"] # [doc = " expression."] # [doc = ""] # [doc = " If you don't need error handling and aren't limited by setup time, it is"] # [doc = " also possible to directly use a `&str` as a strategy with the same effect."] pub fn string_regex (regex : & str) -> ParseResult < String > { let hir = ParserBuilder :: new () . build () . parse (regex) ? ; string_regex_parsed (& hir) }
};
}
