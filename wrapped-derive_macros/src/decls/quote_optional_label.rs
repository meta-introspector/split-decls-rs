macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! quote_optional_label {
    () => {
        deps!();
        pub (crate) fn quote_optional_label (label : Option < Label >) -> proc_macro2 :: TokenStream { match label { Some (label) => { let label = quote_label (label) ; quote ! (Some (# label)) } None => quote ! (None) , } }
    };
}

quote_optional_label!();