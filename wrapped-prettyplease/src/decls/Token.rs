macro_rules! Token {
    () => {
        pub enum Token { Group (Delimiter , TokenStream) , Ident (Ident) , Punct (char , Spacing) , Literal (Literal) , }
    };
}

Token!()