macro_rules! quote_optional_tag_owned {
    () => {
        pub (crate) fn quote_optional_tag_owned (tag : Option < & Path >) -> proc_macro2 :: TokenStream { match tag { Some (tag) => quote ! (Some (# tag)) , None => quote ! (None) , } }
    };
}

quote_optional_tag_owned!()