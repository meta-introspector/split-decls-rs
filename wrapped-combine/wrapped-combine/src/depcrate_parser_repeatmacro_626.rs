// Generated macro for macro_626 (macro)
macro_rules! Depcrate_parser_repeatmacro_626 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_626"}
// Dependencies: {}
parser ! { pub struct SkipMany ; type PartialState = < Ignore < Many < Sink , Ignore < P >>> as Parser < Input >>:: PartialState ; # [doc = " Parses `p` zero or more times ignoring the result."] # [doc = ""] # [doc = " NOTE: If `p` can succeed without consuming any input this may hang forever as `skip_many` will"] # [doc = " repeatedly use `p` to parse the same location in the input every time"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let result = skip_many(digit())"] # [doc = "     .parse(\"A\");"] # [doc = " assert_eq!(result, Ok(((), \"A\")));"] # [doc = " # }"] # [doc = " ```"] pub fn skip_many [Input , P] (p : P) (Input) -> () where [P : Parser < Input >,] { ignore (many ::< Sink , _ , _ > (ignore (p))) } }
};
}
