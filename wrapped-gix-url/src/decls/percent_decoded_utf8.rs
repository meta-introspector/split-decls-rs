macro_rules! deps {
    () => {
        UrlKind!();
        Error!();
    };
}

macro_rules! percent_decoded_utf8 {
    () => {
        deps!();
        fn percent_decoded_utf8 (s : & str , kind : UrlKind) -> Result < String , Error > { Ok (percent_decode_str (s) . decode_utf8 () . map_err (| err | Error :: Utf8 { url : s . into () , kind , source : err , }) ? . into_owned ()) }
    };
}

percent_decoded_utf8!();