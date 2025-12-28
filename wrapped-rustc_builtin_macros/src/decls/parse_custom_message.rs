macro_rules! parse_custom_message {
    () => {
        fn parse_custom_message (parser : & mut Parser < '_ >) -> Option < TokenStream > { let ts = parser . parse_tokens () ; if ! ts . is_empty () { Some (ts) } else { None } }
    };
}

parse_custom_message!()