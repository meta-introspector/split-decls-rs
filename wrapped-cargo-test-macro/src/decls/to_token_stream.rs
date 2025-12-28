macro_rules! to_token_stream {
    () => {
        fn to_token_stream (code : & str) -> TokenStream { code . parse () . unwrap () }
    };
}

to_token_stream!()