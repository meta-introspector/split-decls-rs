macro_rules! deps {
    () => {
        FormatArg!();
        Result!();
        SpanError!();
    };
}

macro_rules! get_args_and_format_string {
    () => {
        deps!();
        # [doc = " Retrieves the original format string and arguments given to the public macros."] pub fn get_args_and_format_string (input : TokenStream ,) -> Result < (LitStr , Punctuated < FormatArg , Comma >) , SpanError > { let args = parse_args (input) ? ; let format_string = get_format_string (args . first ()) ? ; Ok ((format_string , args)) }
    };
}

get_args_and_format_string!();