// Generated macro for chainl1 (function)
macro_rules! Depcrate_parser_repeatchainl1 {
() => {
// Module: crate::parser::repeat
// Provides: {"chainl1"}
// Dependencies: {}
# [doc = " Parses `p` 1 or more times separated by `op`. The value returned is the one produced by the"] # [doc = " left associative application of the function returned by the parser `op`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let number = digit().map(|c: char| c.to_digit(10).unwrap());"] # [doc = " let sub = token('-').map(|_| |l: u32, r: u32| l - r);"] # [doc = " let mut parser = chainl1(number, sub);"] # [doc = " assert_eq!(parser.parse(\"9-3-5\"), Ok((1, \"\")));"] # [doc = " # }"] # [doc = " ```"] pub fn chainl1 < Input , P , Op > (parser : P , op : Op) -> Chainl1 < P , Op > where Input : Stream , P : Parser < Input > , Op : Parser < Input > , Op :: Output : FnOnce (P :: Output , P :: Output) -> P :: Output , { Chainl1 (parser , op) }
};
}
