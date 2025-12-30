// Generated macro for get_args_and_format_string (function)
macro_rules! Depcrate_format_argsget_args_and_format_string {
() => {
// Module: crate::format_args
// Provides: {"get_args_and_format_string"}
// Dependencies: {}
# [doc = " Retrieves the original format string and arguments given to the public macros."] pub fn get_args_and_format_string (input : TokenStream ,) -> Result < (LitStr , Punctuated < FormatArg , Comma >) , SpanError > { let args = parse_args (input) ? ; let format_string = get_format_string (args . first ()) ? ; Ok ((format_string , args)) }
};
}
