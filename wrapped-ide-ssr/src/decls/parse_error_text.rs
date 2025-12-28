macro_rules! deps {
    () => {
        SsrRule!();
    };
}

macro_rules! parse_error_text {
    () => {
        deps!();
        fn parse_error_text (query : & str) -> String { format ! ("{}" , query . parse ::< SsrRule > () . unwrap_err ()) }
    };
}

parse_error_text!()