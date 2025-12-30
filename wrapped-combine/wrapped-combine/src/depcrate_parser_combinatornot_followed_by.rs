// Generated macro for not_followed_by (function)
macro_rules! Depcrate_parser_combinatornot_followed_by {
() => {
// Module: crate::parser::combinator
// Provides: {"not_followed_by"}
// Dependencies: {}
# [doc = " Succeeds only if `parser` fails."] # [doc = " Never consumes any input."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::{alpha_num, string};"] # [doc = " # fn main() {"] # [doc = " let result = string(\"let\")"] # [doc = "     .skip(not_followed_by(alpha_num()))"] # [doc = "     .parse(\"letx\")"] # [doc = "     .map(|x| x.0);"] # [doc = " assert!(result.is_err());"] # [doc = ""] # [doc = " # }"] # [doc = " ```"] pub fn not_followed_by < Input , P > (parser : P) -> NotFollowedBy < P > where Input : Stream , P : Parser < Input > , P :: Output : Into < Info < < Input as StreamOnce > :: Token , < Input as StreamOnce > :: Range , & 'static str > > , { NotFollowedBy (parser) }
};
}
