macro_rules! deps {
    () => {
        Error!();
        UrlKind!();
    };
}

macro_rules! input_to_utf8 {
    () => {
        deps!();
        fn input_to_utf8 (input : & BStr , kind : UrlKind) -> Result < & str , Error > { std :: str :: from_utf8 (input) . map_err (| source | Error :: Utf8 { url : input . to_owned () , kind , source , }) }
    };
}

input_to_utf8!()