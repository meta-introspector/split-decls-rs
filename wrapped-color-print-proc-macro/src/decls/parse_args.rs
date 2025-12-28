macro_rules! deps {
    () => {
        SpanError!();
        Result!();
        FormatArg!();
        Error!();
    };
}

macro_rules! parse_args {
    () => {
        deps!();
        # [doc = " Parses the arguments of a `format!`-like macro."] pub fn parse_args (input : TokenStream) -> Result < Punctuated < FormatArg , Comma > , SpanError > { let parser = Punctuated :: < FormatArg , Token ! [,] > :: parse_terminated ; parser . parse (input) . map_err (| e | Error :: Parse (e . to_string ()) . into ()) }
    };
}

parse_args!()