// Generated macro for parse_style (function)
macro_rules! Depcrate_log_format_parserparse_style {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_style"}
// Dependencies: {}
fn parse_style (input : & str) -> IResult < & str , IntermediateOutput , () > { let mut parse_type = map_res (take_while (char :: is_alphabetic) , move | s | { let style = match s { "bold" => colored :: Styles :: Bold , "italic" => colored :: Styles :: Italic , "underline" => colored :: Styles :: Underline , "strike" => colored :: Styles :: Strikethrough , "dimmed" => colored :: Styles :: Dimmed , _ => return Err (()) , } ; Ok (IntermediateOutput :: Style (style)) }) ; parse_type . parse (input) }
};
}
