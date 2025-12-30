// Generated macro for input_to_utf8 (function)
macro_rules! Depcrate_parseinput_to_utf8 {
() => {
// Module: crate::parse
// Provides: {"input_to_utf8"}
// Dependencies: {}
fn input_to_utf8 (input : & BStr , kind : UrlKind) -> Result < & str , Error > { std :: str :: from_utf8 (input) . map_err (| source | Error :: Utf8 { url : input . to_owned () , kind , source , }) }
};
}
