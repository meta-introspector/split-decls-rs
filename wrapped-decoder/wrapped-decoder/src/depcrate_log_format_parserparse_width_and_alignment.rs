// Generated macro for parse_width_and_alignment (function)
macro_rules! Depcrate_log_format_parserparse_width_and_alignment {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_width_and_alignment"}
// Dependencies: {}
fn parse_width_and_alignment (input : & str) -> IResult < & str , IntermediateOutput , () > { let (input , alignment) = opt (map_res (one_of ("<^>") , move | c | match c { '^' => Ok (Alignment :: Center) , '<' => Ok (Alignment :: Left) , '>' => Ok (Alignment :: Right) , _ => Err (()) , })) (input) ? ; let (input , width) = digit1 . parse (input) ? ; let padding = if width . starts_with ('0') { Padding :: Zero } else { Padding :: Space } ; let Ok (width) = width . parse :: < usize > () else { return Err (nom :: Err :: Error (())) ; } ; Ok ((input , IntermediateOutput :: WidthAndAlignment ((width , padding , alignment)) ,)) }
};
}
