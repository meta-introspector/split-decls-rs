macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! quote_label {
    () => {
        deps!();
        pub (crate) fn quote_label (label : Label) -> proc_macro2 :: TokenStream { match label { Label :: Implicit (implicit) => { quote ! (& sval :: Label :: new (# implicit) . with_tag (& sval :: tags :: VALUE_IDENT)) } Label :: Const (explicit) => quote ! (& sval :: Label :: new (# explicit)) , Label :: Ident (explicit) => quote ! (&# explicit) , } }
    };
}

quote_label!()