macro_rules! deps {
    () => {
        GeneratorResult!();
        GeneratorError!();
    };
}

macro_rules! generate_default_with {
    () => {
        deps!();
        fn generate_default_with (lit : & LitStr) -> GeneratorResult < TokenStream > { let str = lit . value () ; let tokens : TokenStream = str . parse () . map_err (| err | GeneratorError :: Syn (syn :: Error :: from (err))) ? ; Ok (quote ! { (# tokens) }) }
    };
}

generate_default_with!();