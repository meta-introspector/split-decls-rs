// Generated macro for look_ahead (function)
macro_rules! Depcrate_parser_combinatorlook_ahead {
() => {
// Module: crate::parser::combinator
// Provides: {"look_ahead"}
// Dependencies: {}
# [doc = " `look_ahead(p)` acts as `p` but doesn't consume input on success."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::string;"] # [doc = " # fn main() {"] # [doc = " let mut p = look_ahead(string(\"test\"));"] # [doc = ""] # [doc = " let result = p.parse(\"test str\");"] # [doc = " assert_eq!(result, Ok((\"test\", \"test str\")));"] # [doc = ""] # [doc = " let result = p.parse(\"aet\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn look_ahead < Input , P > (p : P) -> LookAhead < P > where Input : Stream , P : Parser < Input > , { LookAhead (p) }
};
}
