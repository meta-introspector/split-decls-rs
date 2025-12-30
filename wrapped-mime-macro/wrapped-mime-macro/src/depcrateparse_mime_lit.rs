// Generated macro for parse_mime_lit (function)
macro_rules! Depcrateparse_mime_lit {
() => {
// Module: crate
// Provides: {"parse_mime_lit"}
// Dependencies: {}
fn parse_mime_lit (value : & str) -> Result < mime_parse :: Mime , String > { let mime = mime_parse :: Parser :: cannot_range () . parse (value) ; match mime { Ok (mime) => match mime . private_params_source () { mime_parse :: ParamSource :: None | mime_parse :: ParamSource :: Utf8 (_) => Ok (mime) , mime_parse :: ParamSource :: One (..) => Ok (mime) , _ => Err ("multiple parameters not supported yet" . into ()) } , Err (err) => { Err (format ! ("invalid MediaType: {}" , err)) } } }
};
}
