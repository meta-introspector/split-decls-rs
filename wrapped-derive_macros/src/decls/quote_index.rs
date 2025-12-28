macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! quote_index {
    () => {
        deps!();
        pub (crate) fn quote_index (index : Index) -> proc_macro2 :: TokenStream { match index { Index :: Explicit (index) => quote ! (& sval :: Index :: from (# index)) , Index :: Implicit (index) => { quote ! (& sval :: Index :: from (# index) . with_tag (& sval :: tags :: VALUE_OFFSET)) } } }
    };
}

quote_index!()