// Generated macro for percent_decoded_utf8 (function)
macro_rules! Depcrate_parsepercent_decoded_utf8 {
() => {
// Module: crate::parse
// Provides: {"percent_decoded_utf8"}
// Dependencies: {}
fn percent_decoded_utf8 (s : & str , kind : UrlKind) -> Result < String , Error > { Ok (percent_decode_str (s) . decode_utf8 () . map_err (| err | Error :: Utf8 { url : s . into () , kind , source : err , }) ? . into_owned ()) }
};
}
