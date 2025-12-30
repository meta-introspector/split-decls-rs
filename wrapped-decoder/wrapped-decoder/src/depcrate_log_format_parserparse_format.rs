// Generated macro for parse_format (function)
macro_rules! Depcrate_log_format_parserparse_format {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_format"}
// Dependencies: {}
fn parse_format < const FAIL_ON_ERR : bool > (input : & str) -> IResult < & str , IntermediateOutput , () > { let result = alt ((parse_color , parse_style , parse_width_and_alignment)) . parse (input) ; if ! FAIL_ON_ERR { result } else { match result { Ok (r) => Ok (r) , Err (_) => Err (nom :: Err :: Failure (())) , } } }
};
}
