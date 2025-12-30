// Generated macro for chainr1 (function)
macro_rules! Depcrate_parser_repeatchainr1 {
() => {
// Module: crate::parser::repeat
// Provides: {"chainr1"}
// Dependencies: {}
# [doc = " Parses `p` one or more times separated by `op`. The value returned is the one produced by the"] # [doc = " right associative application of the function returned by `op`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let number = digit().map(|c: char| c.to_digit(10).unwrap());"] # [doc = " let pow = token('^').map(|_| |l: u32, r: u32| l.pow(r));"] # [doc = " let mut parser = chainr1(number, pow);"] # [doc = "     assert_eq!(parser.parse(\"2^3^2\"), Ok((512, \"\")));"] # [doc = " }"] # [doc = " ```"] pub fn chainr1 < Input , P , Op > (parser : P , op : Op) -> Chainr1 < P , Op > where Input : Stream , P : Parser < Input > , Op : Parser < Input > , Op :: Output : FnOnce (P :: Output , P :: Output) -> P :: Output , { Chainr1 (parser , op) }
};
}
