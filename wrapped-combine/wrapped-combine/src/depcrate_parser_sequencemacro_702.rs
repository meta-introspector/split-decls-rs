// Generated macro for macro_702 (macro)
macro_rules! Depcrate_parser_sequencemacro_702 {
() => {
// Module: crate::parser::sequence
// Provides: {"macro_702"}
// Dependencies: {}
parser ! { # [derive (Copy , Clone)] pub struct Between ; type PartialState = < Map < (L , P , R) , fn ((L :: Output , P :: Output , R :: Output)) -> P :: Output > as Parser < Input >>:: PartialState ; # [doc = " Parses `open` followed by `parser` followed by `close`."] # [doc = " Returns the value of `parser`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::string;"] # [doc = " # fn main() {"] # [doc = " let result = between(token('['), token(']'), string(\"rust\"))"] # [doc = "     .parse(\"[rust]\")"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(\"rust\"));"] # [doc = " # }"] # [doc = " ```"] pub fn between [Input , L , R , P] (open : L , close : R , parser : P) (Input) -> P :: Output where [Input : Stream , L : Parser < Input >, R : Parser < Input >, P : Parser < Input >,] { fn middle < T , U , V > ((_ , x , _) : (T , U , V)) -> U { x } (open , parser , close) . map (middle) } }
};
}
