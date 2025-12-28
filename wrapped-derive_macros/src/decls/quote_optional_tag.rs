macro_rules! quote_optional_tag {
    () => {
        pub (crate) fn quote_optional_tag (tag : Option < & Path >) -> proc_macro2 :: TokenStream { match tag { Some (tag) => quote ! (Some (&# tag)) , None => quote ! (None) , } }
    };
}

quote_optional_tag!()