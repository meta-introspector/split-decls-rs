macro_rules! deps {
    () => {
        UrlKind!();
        ParsedUrl!();
        Error!();
    };
}

macro_rules! url_user {
    () => {
        deps!();
        fn url_user (url : & crate :: simple_url :: ParsedUrl < '_ > , kind : UrlKind) -> Result < Option < String > , Error > { if url . username . is_empty () && url . password . is_none () { Ok (None) } else { Ok (Some (percent_decoded_utf8 (url . username , kind) ?)) } }
    };
}

url_user!()