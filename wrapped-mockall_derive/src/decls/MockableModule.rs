macro_rules! MockableModule {
    () => {
        pub (crate) struct MockableModule { pub attrs : TokenStream , pub vis : Visibility , pub mock_ident : Ident , # [doc = " Ident of the original module, if any"] pub orig_ident : Option < Ident > , pub content : Vec < Item > }
    };
}

MockableModule!();