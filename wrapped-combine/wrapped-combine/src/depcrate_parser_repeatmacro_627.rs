// Generated macro for macro_627 (macro)
macro_rules! Depcrate_parser_repeatmacro_627 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_627"}
// Dependencies: {}
parser ! { pub struct SkipMany1 ; type PartialState = < Ignore < Many1 < Sink , Ignore < P >>> as Parser < Input >>:: PartialState ; # [doc = " Parses `p` one or more times ignoring the result."] # [doc = ""] # [doc = " NOTE: If `p` can succeed without consuming any input this may hang forever as `skip_many1` will"] # [doc = " repeatedly use `p` to parse the same location in the input every time"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let result = skip_many1(digit())"] # [doc = "     .parse(\"123A\");"] # [doc = " assert_eq!(result, Ok(((), \"A\")));"] # [doc = " # }"] # [doc = " ```"] pub fn skip_many1 [Input , P] (p : P) (Input) -> () where [P : Parser < Input >,] { ignore (many1 ::< Sink , _ , _ > (ignore (p))) } }
};
}
