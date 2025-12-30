// Generated macro for parse_args (function)
macro_rules! Depcrate_format_argsparse_args {
() => {
// Module: crate::format_args
// Provides: {"parse_args"}
// Dependencies: {}
# [doc = " Parses the arguments of a `format!`-like macro."] pub fn parse_args (input : TokenStream) -> Result < Punctuated < FormatArg , Comma > , SpanError > { let parser = Punctuated :: < FormatArg , Token ! [,] > :: parse_terminated ; parser . parse (input) . map_err (| e | Error :: Parse (e . to_string ()) . into ()) }
};
}
