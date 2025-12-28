macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! quote_optional_index {
    () => {
        deps!();
        pub (crate) fn quote_optional_index (index : Option < Index >) -> proc_macro2 :: TokenStream { match index { Some (index) => { let index = quote_index (index) ; quote ! (Some (# index)) } None => quote ! (None) , } }
    };
}

quote_optional_index!();