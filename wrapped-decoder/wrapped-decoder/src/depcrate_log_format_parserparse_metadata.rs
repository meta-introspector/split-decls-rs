// Generated macro for parse_metadata (function)
macro_rules! Depcrate_log_format_parserparse_metadata {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_metadata"}
// Dependencies: {}
fn parse_metadata (input : & str) -> IResult < & str , IntermediateOutput , () > { let mut parse_type = map_res (take_while (char :: is_alphabetic) , move | s | { let metadata = match s { "c" => LogMetadata :: CrateName , "F" => LogMetadata :: FilePath , "l" => LogMetadata :: LineNumber , "s" => LogMetadata :: Log , "L" => LogMetadata :: LogLevel , "m" => LogMetadata :: ModulePath , "t" => LogMetadata :: Timestamp , _ => { if ! s . is_empty () && s == "f" . repeat (s . len ()) { LogMetadata :: FileName (s . len () as u8) } else { return Err (()) ; } } } ; Ok (IntermediateOutput :: Metadata (metadata)) }) ; parse_type . parse (input) }
};
}
