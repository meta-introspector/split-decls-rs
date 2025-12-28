macro_rules! deps {
    () => {
        MockItemContent!();
    };
}

macro_rules! MockItemModule {
    () => {
        deps!();
        pub (crate) struct MockItemModule { attrs : TokenStream , vis : Visibility , mock_ident : Ident , orig_ident : Option < Ident > , content : Vec < MockItemContent > }
    };
}

MockItemModule!()