// Generated macro for get_format_string (function)
macro_rules! Depcrate_format_argsget_format_string {
() => {
// Module: crate::format_args
// Provides: {"get_format_string"}
// Dependencies: {}
# [doc = " Gets the format string."] pub fn get_format_string (arg : Option < & FormatArg >) -> Result < LitStr , SpanError > { match arg { Some (FormatArg { expr : Expr :: Lit (ExprLit { lit : Lit :: Str (s) , .. }) , .. }) => Ok (s . to_owned ()) , Some (bad_arg) => { Err (SpanError :: new (Error :: MustBeStringLiteral , Some (bad_arg . span ()) ,)) } None => Ok (util :: literal_string ("")) } }
};
}
