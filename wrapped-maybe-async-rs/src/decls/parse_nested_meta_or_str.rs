macro_rules! parse_nested_meta_or_str {
    () => {
        fn parse_nested_meta_or_str (input : ParseStream) -> Result < TokenStream2 > { if let Some (s) = input . parse :: < Option < LitStr > > () ? { let tokens = s . value () . parse () ? ; Ok (tokens) } else { let meta : Meta = input . parse () ? ; Ok (quote ! (# meta)) } }
    };
}

parse_nested_meta_or_str!();