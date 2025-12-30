// Generated macro for parse_color (function)
macro_rules! Depcrate_log_format_parserparse_color {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_color"}
// Dependencies: {}
fn parse_color (input : & str) -> IResult < & str , IntermediateOutput , () > { let mut parse_type = map_res (take_while (char :: is_alphabetic) , move | s | { let color = match s { "severity" => LogColor :: SeverityLevel , "werror" => LogColor :: WarnError , s => match colored :: Color :: from_str (s) { Ok (c) => LogColor :: Color (c) , Err (()) => return Err (()) , } , } ; Ok (IntermediateOutput :: Color (color)) }) ; parse_type . parse (input) }
};
}
