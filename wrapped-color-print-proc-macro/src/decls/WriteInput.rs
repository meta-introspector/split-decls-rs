macro_rules! WriteInput {
    () => {
        struct WriteInput { dst : Expr , rest : TokenStream , }
    };
}

WriteInput!()