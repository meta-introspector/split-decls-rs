// Generated macro for no_partial (function)
macro_rules! Depcrate_parser_combinatorno_partial {
() => {
// Module: crate::parser::combinator
// Provides: {"no_partial"}
// Dependencies: {}
# [doc = " Wraps a parser `p` and disables partial parsing for it (changing the `PartialState` to `()`)"] # [doc = ""] # [doc = " Partial parsing lets a parser accept incomplete (partial) input resume parsing when more input is available without re-parsing any part of the input."] # [doc = " By disabling partial parsing for a parser it will need to restart its parse from the beginning once more input is available (no_partial ONLY affects the wrapped parser,"] # [doc = " any parsers calling the `no_partial` parser will still employ partial parsing)."] # [doc = ""] # [doc = " If you are not using partial parsing this has no effect (except changing the typing of the parser)."] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use]"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::combinator::no_partial;"] # [doc = " # use combine::parser::char::letter;"] # [doc = " # use combine::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     (no_partial(letter()), letter()).easy_parse(\"ab\"),"] # [doc = "     Ok((('a', 'b'), \"\"))"] # [doc = " );"] # [doc = ""] # [doc = " # }"] # [doc = " ```"] pub fn no_partial < Input , P > (p : P) -> NoPartial < P > where Input : Stream , P : Parser < Input > , { NoPartial (p) }
};
}
