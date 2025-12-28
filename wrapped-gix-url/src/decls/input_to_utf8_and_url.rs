macro_rules! deps {
    () => {
        Error!();
        Url!();
        ParsedUrl!();
        UrlParseError!();
        UrlKind!();
    };
}

macro_rules! input_to_utf8_and_url {
    () => {
        deps!();
        fn input_to_utf8_and_url (input : & BStr , kind : UrlKind) -> Result < (& str , crate :: simple_url :: ParsedUrl < '_ >) , Error > { let input = input_to_utf8 (input , kind) ? ; crate :: simple_url :: ParsedUrl :: parse (input) . map (| url | (input , url)) . map_err (| source | { match source { crate :: simple_url :: UrlParseError :: RelativeUrlWithoutBase => { Error :: RelativeUrl { url : input . to_owned () } } _ => Error :: Url { url : input . to_owned () , kind , source , } , } }) }
    };
}

input_to_utf8_and_url!();