macro_rules! DepTableInput {
    () => {
        struct DepTableInput { name : LitStr , _comma_token : Token ! [,] , table_content : proc_macro2 :: TokenStream , }
    };
}

DepTableInput!();