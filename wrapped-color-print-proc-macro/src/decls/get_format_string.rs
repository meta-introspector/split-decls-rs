macro_rules! deps {
    () => {
        Error!();
        FormatArg!();
        Result!();
        SpanError!();
    };
}

macro_rules! get_format_string {
    () => {
        deps!();
        # [doc = " Gets the format string."] pub fn get_format_string (arg : Option < & FormatArg >) -> Result < LitStr , SpanError > { match arg { Some (FormatArg { expr : Expr :: Lit (ExprLit { lit : Lit :: Str (s) , .. }) , .. }) => Ok (s . to_owned ()) , Some (bad_arg) => { Err (SpanError :: new (Error :: MustBeStringLiteral , Some (bad_arg . span ()) ,)) } None => Ok (util :: literal_string ("")) } }
    };
}

get_format_string!();